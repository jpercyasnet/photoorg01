use iced::alignment::{Alignment};
// use iced::widget::scrollable::Properties;
use iced::theme::{Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text, horizontal_space,
    image, container, Column, Row, text_input, Space, Radio, combo_box,
};
use iced::event::{self, Event};
use iced::Subscription;
use iced::window;
use iced::{Element};
use iced::{Center, Color, Task, Length, Size};

use serde::{Deserialize, Serialize};

extern crate image as create_image;
mod get_winsize;
mod dump_file;
mod fromdirpressm;
mod copypressm;
mod todirpressm;
mod get_fromdirlistm;
// mod gen_merge;
// mod dateinname_merge;
// mod celldatename_merge;
// mod todirrefreshm;
// mod get_prevafterm;

use get_fromdirlistm::get_fromdirlistm;
use get_winsize::get_winsize;
use fromdirpressm::fromdirpressm;
use todirpressm::todirpressm;
use copypressm::copypressm;
// use todirrefreshm::todirrefreshm;
// use get_prevafterm::get_prevafterm;

pub fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, _errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
//         println!("{}", errstring);
//     } else {
//         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(ImageList::title, ImageList::update, ImageList::view)
        .window_size((widthxx, heightxx))
        .theme(ImageList::theme)
        .subscription(ImageList::subscription)
        .run_with(ImageList::new)

}

#[derive(Debug)]
enum ImageList {
    Loaded(State),
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YearChoice {
    NON,
    YR1,
    YR2,
    YR3,
}
#[derive(Debug, Clone)]
enum Monthval {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Default for YearChoice {
    fn default() -> Self {
        YearChoice::NON
    }
}


#[derive(Debug, Default)]
struct State {
    filter: Filter,
    filterf: Filterf,
    images: Vec<ImageItem>,
    files: Vec<File>,
    yearchoice_value: YearChoice,
    fromdir_value: String,
    todir_value: String,
    msg_value: String,
    mess_color: Color,
    size_value: String,
    fromyear_value: String,
    toyear_value: String,
    months: combo_box::State<Monthval>,
    month_value: Option<Monthval>,
    screenwidth: f32,
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    FilterChangedf(Filterf),
    ImageMessage(usize, ImageMessage),
    FileMessage(usize, FileMessage),
    FromDirPressed,
    ToDirPressed,
    RefreshPressed,
    YearRadioSelected(YearChoice),
    CopyPressed,
    MonthSelected(Monthval),
    SizeChanged(String),
    FromYear(String),
    ToYear(String),
    Size(Size),

}

impl ImageList {
    fn new() -> (Self, Task<Message>) {
        let mut widthxx: u32 = 1300;
        let (errcode, errstring, widtho, _heighto) = get_winsize();
        let for_message: String;
        if errcode == 0 {
            widthxx = widtho;
            for_message = format!("{}", errstring);
        } else {
            for_message = format!("**ERROR {} get_winsize: {}", errcode, errstring);
        }

        (
            ImageList::Loaded(State
               {
                filter:Filter::All,
                filterf:Filterf::All,
                images:Vec::<ImageItem>::new(),
                files:Vec::<File>::new(),
                fromdir_value: "no directory".to_string(),
                todir_value: "no directory".to_string(),
                yearchoice_value:YearChoice::NON,
                mess_color: Color::from([0.5, 0.5, 1.0]),
                msg_value: for_message.to_string(),
                size_value: "140".to_string(),
                month_value: Some(Monthval::Jan),
                months: iced::widget::combo_box::State::<Monthval>,
                fromyear_value: "2025".to_string(),
                toyear_value: "2025".to_string(),
                screenwidth: widthxx as f32,
                }
            ),
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Copy images into a directory -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            ImageList::Loaded(state) => {

                let command = match message {
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Task::none()
                    }
                    Message::FilterChangedf(filterf) => {
                        state.filterf = filterf;

                        Task::none()
                    }
                    Message::ImageMessage(i, image_message) => {
                        if let Some(image) = state.images.get_mut(i) {

                            image.update(image_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    Message::FileMessage(i, file_message) => {
                        if let Some(file) = state.files.get_mut(i) {

                            file.update(file_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    Message::Size(size) => {
                         state.screenwidth = size.width;
                         Task::none()
                    }
                    Message::MonthSelected(monthval) => {
                         state.month_value = Some(monthval);
                         Task::none()
                    }
                    Message::YearRadioSelected(xchoice) => {
                        let strx = match xchoice {
                        YearChoice::NON => "choice no year selected",
                        YearChoice::YR1 => "choice year 1 selected",
                        YearChoice::YR2 => "choice year 2 selected",
                        YearChoice::YR3 => "choice year 3 selected" };
                       state.yearchoice_value = xchoice;
                       state.msg_value = strx.to_string();
                       Task::none()
                    }

                    Message::FromDirPressed => {
                       let (errcode, errstr, newdir, listitems, newtoi, icon_int1) = fromdirpressm(state.fromdir_value.clone(), state.size_value.clone());
                       if errcode == 0 {
                           if newtoi != 0 {
                               state.images.clear();                         
                               for indexi in 0..newtoi {
                                    state.fromdir_value = newdir.to_string();
                                    let linestr = listitems[indexi as usize].clone();
                                    let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
                                    let filefromx = lineparse[0].to_string();
                                    let fullpath = state.fromdir_value.clone() + "/" + &filefromx;
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        state
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                    } else {
                                        println!("no wxh fullpath -{}- ", fullpath);
                                    }
                               }
                           }
                       }
                       state.msg_value = errstr.to_string();
                       if errcode == 0 {
                           state.mess_color = Color::from([0.0, 1.0, 0.0]);
                       } else {
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };

                       Task::none()
                    } 
                    Message::ToDirPressed => {
                        let (errcode, errstr, newdir) = todirpressm(state.fromdir_value.clone());
                        state.msg_value = errstr.to_string();
                        if errcode == 0 {
                            state.todir_value = newdir;
                            state.mess_color = Color::from([0.0, 1.0, 0.0]);
                        } else {
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        }
                        Task::none()
                    } 
                    Message::CopyPressed => {
// check if selections
                       let images_selected = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_selected < 1 {
                           state.msg_value = "no FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else if images_selected > 1 {
                           state.msg_value = "more than 1 FROM image selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let files_selected = state.files.iter().filter(|fileitem| fileitem.yearset).count();
                           if files_selected < 1 {
                               state.msg_value = "no TO image selected".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else if files_selected > 1 {
                               state.msg_value = "more than 1 TO image selected".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           } else {
                               let mut fromimagestr: String = " ".to_string();
                               for imagesy in state.images.iter() {
                                    if imagesy.completed {
                                       fromimagestr = imagesy.description.clone();
                                    }
                               }
                               let mut toimagestr: String = " ".to_string();
                               for filesy in state.files.iter() {
                                    if filesy.yearset {
                                       toimagestr = filesy.description.clone();
                                    }
                               }

                               let (errcode, errstr) = copypressm(fromimagestr, toimagestr, state.fromdir_value.clone(),
                                                                state.todir_value.clone());
                               if errcode == 0 {
                                   state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                   state.msg_value = errstr.to_string();
                               } else {
                                   state.msg_value = errstr.to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               }                                
                           }
                       };
                       Task::none()
                    }
                    Message::RefreshPressed => {
                       if state.fromyear_value.len() == 0 { 
                           state.msg_value = "********* from year has no value **********".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       } else {
                           let from_int: i32 = state.fromyear_value.parse().unwrap_or(-99);
                           if from_int > 0 {
                               if (from_int < 1800) | (from_int > 2100) {
                                   state.msg_value = "********* from year not between 1800 and 2100 **********".to_string();
                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               } else {
                                   if state.toyear_value.len() == 0 { 
                                       state.msg_value = "********* to year has no value **********".to_string();
                                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                   } else {
                                       let mut to_int: i32 = state.toyear_value.parse().unwrap_or(-99);
                                       if to_int > 0 {
                                           if (to_int < 1800) | (to_int > 2100) {
                                               state.msg_value = "********* from year not between 1800 and 2100 **********".to_string();
                                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                           } else {
                                               if to_int < from_int {
                                                   state.msg_value = "********* to year less than from year **********".to_string();
                                                   state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                               } else {
                                                   state.files.clear(); 
                                                   to_int = to_int + 1;
                                                   for yearnum in from_int..to_int {
                                                        state
                                                             .files
                                                             .push(File::new(format!("{}", yearnum)));
                                                   }
                                                   state.msg_value = "********* refreshed years **********".to_string();
                                                   state.mess_color = Color::from([0.0, 1.0, 0.0]);
                                               }
                                           }
                                       } else {
                                           state.msg_value = "********* to year has bad value **********".to_string();
                                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                                       }
                                   }
                               }
                           } else {
                               state.msg_value = "********* from year has bad value **********".to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       }
                       Task::none()
                    } 
                    Message::SizeChanged(value) => { state.size_value = value; Task::none() }
                    Message::FromYear(value) => { state.fromyear_value = value; Task::none() }
                    Message::ToYear(value) => { state.toyear_value = value; Task::none() }

                };

                Task::batch(vec![command, Task::none()])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            ImageList::Loaded(State {
                filter,
                filterf,
                files,
                images,
                fromdir_value,
                todir_value,
                yearchoice_value,
                msg_value,
                mess_color,
                size_value,
                fromyear_value,
                month_value,
                months,
                toyear_value,
                screenwidth,
                ..
            }) => {
                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(20),
                 text(msg_value).size(20).color(*mess_color),
            ].align_y(Alignment::Center).spacing(10).padding(5)
                    ));

                let mut dirbutshow = Column::new().spacing(10);
                let dirspace = 5.0;
//                if fromdir_value.len()*8 < 600 {
//                    dirspace = 600.0 - 8.0*fromdir_value.len() as f32;
//                }
                dirbutshow = dirbutshow.push(container(row![container(row![button("From Directory Button")
                                                             .on_press(Message::FromDirPressed),
                                                            text(fromdir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                             Space::with_width(Length::Fixed(dirspace)),
                                                             container(row![button("To Directory Button")
                                                             .on_press(Message::ToDirPressed),
                                                            text(todir_value)
                                                             .size(20)].spacing(10)).width(Length::Fill),
                                                           ].align_y(Alignment::Center).spacing(10).padding(1),
                 ));
                let controls = view_controls(images, *filter);
                let filtered_images =
                    images.iter().filter(|imageitem| filter.matches(imageitem));

                let mut imagescol1 = Column::new().spacing(10);
                let mut imagescol2 = Column::new().spacing(10);
                let mut imagescol3 = Column::new().spacing(10);
                let mut colpos = 0;
                let mut n = 0;
                if filtered_images.clone().count() == 0 {
                    n = 1;
                    imagescol1 = imagescol1.push(container(row![empty_message(match filter {
                        Filter::All => "No directory selected or no files in directory",
                        Filter::Active => "All files have been selected",
                        Filter::Selected => "No files have been selected" 
                    })]));
                } else {
                    for imagesy in images.iter() {
                         if imagesy.completed {
                             if (filter == &Filter::All) || (filter == &Filter::Selected) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 2;
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                            }
                         } else {
                             if (filter == &Filter::All) || (filter == &Filter::Active) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 2;
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                              }
                           }
                         }
                         n = n + 1;
                    }
                }
                let mut imagesrow = Row::new().spacing(20);
                imagesrow = imagesrow.push(container(imagescol1).padding(10).width(Length::Fixed(300.0)));
                if n > 1 {
                    imagesrow = imagesrow.push(container(imagescol2).padding(10).width(Length::Fixed(300.0)));
                }
                if n > 2 {
                    imagesrow = imagesrow.push(container(imagescol3).padding(10).width(Length::Fixed(300.0)));
                }

                let scrollable_content: Element<Message> =
                  Element::from(scrollable(
                    imagesrow
                )
                .height(Length::Fill)
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);
//                        .anchor(self.anchor);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 

//                let controlsf = view_controlsf(files, *filterf);
                let files_left = files.iter().filter(|file| file.yearset).count();
                let mut file_text = "no year selected".to_string();
                if files_left > 1 {
                    file_text = format!("too many years selected: {}", files_left);
                } else if files_left == 1 {
                    file_text = "year selected!".to_string();
                }
                let controlsf = row![text(file_text).size(20)].spacing(10).padding(10);

                let filtered_files =
                    files.iter().filter(|file| filterf.matches(file));

                let mut filescol1 = Column::new().spacing(10);
                let mut n = 0;
                if filtered_files.clone().count() == 0 {
                    filescol1 = filescol1.push(container(row![empty_message(match filterf {
                        Filterf::All => "No directory selected or no files in directory",
                        Filterf::Active => "All files have been selected",
                        Filterf::Yearset => "No files have been selected" 
                    })]));
                } else {
                    for filesy in files.iter() {
                         if filesy.yearset {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Yearset) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         } else {
                             if (filterf == &Filterf::All) || (filterf == &Filterf::Active) {
                                 filescol1 = filescol1.push(container(row![filesy.view(n).map(move |message| {
                                    Message::FileMessage(n, message)
                                   })]));
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut filesrow = Row::new().spacing(20);
                filesrow = filesrow.push(container(filescol1).padding(10).width(Length::Fixed(500.0)));

                let scrollable_contentf: Element<Message> =
                  Element::from(scrollable(
                    filesrow
                )
                .height(Length::Fill)
                .width(Length::Fixed(500.0))
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);
//                        .anchor(self.anchor);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 


           
                let contentab = row![text(" Icon Size: ").size(20),
                                     text_input("140", size_value).on_input(Message::SizeChanged).padding(10).size(20).width(80),
                                     horizontal_space(), 
                                     button("Refresh").on_press(Message::RefreshPressed).padding(10),
                                     text(" Year Range: ").size(20),
                                     text_input("2025", fromyear_value).on_input(Message::FromYear).padding(10).size(20).width(80),
                                     text(" to ").size(20),
                                     text_input("2025", toyear_value).on_input(Message::ToYear).padding(10).size(20).width(80),
                                     Space::with_width(Length::Fixed(80.0)),
                                     button("Copy").on_press(Message::CopyPressed).padding(10),
                                    ].spacing(10).padding(10);

                let winwidth: f32 = screenwidth - 20.0;

                let columnfrom = column![text("  ********* FROM *********"), controls, scrollable_content].width(Length::Fill);

                let selected_yearchoice = Some(yearchoice_value);
                let ya = Radio::new(
                         "No year",
                         YearChoice::NON,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
                let yb = Radio::new(
                         "Year 1",
                         YearChoice::YR1,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
           
                let yc = Radio::new(
                         "Year 2",
                         YearChoice::YR2,
                         selected_yearchoice.copied(),
                         Message::YearRadioSelected,
                ).size(15);
           
                let yd = Radio::new(
                           "Year 3",
                           YearChoice::YR3,
                           selected_yearchoice.copied(),
                           Message::YearRadioSelected
                ).size(15);
                
                 let pickmonth = combo_box(
                                      &months,
                                      "Select the month...",
                                      month_value.as_ref(),
                                      Message::MonthSelected
                                    )
                                    .into();

                  let columntob = column![row![ya, yb, yc, yd, pickmonth].spacing(20), controlsf, scrollable_contentf].width(Length::Fill);


                   column![messcol, dirbutshow, contentab, row![columnfrom, columntob]]
                         .spacing(1)
                         .max_width(winwidth)
                         .padding(10)
                         .into()
            }
        }
    }
    fn theme(&self) -> Theme {
         Theme::Dracula
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Window(window::Event::Resized(size)) => {
                Some(Message::Size(size))
            }
            _ => None,
        })
    }

}

impl std::fmt::Display for Monthval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Jan => "Jan",
            Self::Feb => "Feb",
            Self::Mar => "Mar",
            Self::Apr => "Apr",
            Self::May => "May",
            Self::Jun => "Jun",
            Self::Jul => "Jul",
            Self::Aug => "Aug",
            Self::Sep => "Sep",
            Self::Oct => "Oct",
            Self::Nov => "Nov",
            Self::Dec => "Dec",
        })
    }
}



#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Selected(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32,) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
        }
    }

    fn update(&mut self, message: ImageMessage) {
        match message {
            ImageMessage::Selected(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self, _i: usize) -> Element<ImageMessage> {
        let checkbox = checkbox(
            &self.description,
            self.completed).on_toggle(ImageMessage::Selected).width(Length::Fill).text_size(12);
        let newimage = image::Handle::from_rgba(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 
        let colhigh: f32;
        if self.twidth > self.theight {
            colhigh = self.twidth as f32 + 50.0;
        } else {
            colhigh = self.theight as f32 + 50.0;
        }
        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fill)
                 .width(Length::Fill),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_x(Alignment::Center)
        .height(Length::Fixed(colhigh))
        .spacing(5)
        .into()

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[derive(Debug, Clone)]
struct File {
    description: String,
    yearset: bool,

}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Yearset(bool),
}

impl File {
    fn new(description: String) -> Self {
        File {
            description,
            yearset: false,
        }
    }
    fn update(&mut self, message: FileMessage) {
        match message {
            FileMessage::Yearset(yearset) => {
                self.yearset = yearset;
            }

        }
    }
    fn view(&self, _i: usize) -> Element<FileMessage> {
                let checkbox = checkbox(
                    &self.description,
                    self.yearset).on_toggle(FileMessage::Yearset).width(Length::Fixed(500.0));
                row![
                    checkbox,
                ]
                .spacing(20)
                .align_y(Alignment::Center)
                .into()
    }
}


fn view_controlsf(files: &[File], current_filter: Filterf) -> Element<Message> {
    let files_left = files.iter().filter(|file| file.yearset).count();
    let filter_button = |label, filterf, current_filter| {
        let label = text(label).size(16);
        let button = button(label).style(if filterf == current_filter {
            button::primary
        } else {
            button::text
        });
        button.on_press(Message::FilterChangedf(filterf)).padding(8)
    };
        row![Space::with_width(Length::Fixed(20.0)),
            text(format!(
            "{} {} selected",
            files_left,
            if files_left == 1 { "file" } else { "files" }
        ))
        .size(16),
            filter_button("All", Filterf::All, current_filter),
            filter_button("Not Selected", Filterf::Active, current_filter),
            filter_button("Selected", Filterf::Yearset, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    .align_y(Alignment::Center)
    .padding(10)
    .into()
}

fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![Space::with_width(Length::Fixed(20.0)),
        text(format!(
            "{} {} selected",
            images_left,
            if images_left == 1 { "file" } else { "files" }
        ))
        .size(16),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Selected", Filter::Active, current_filter),
            filter_button("Selected", Filter::Selected, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Selected,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, imageitem: &ImageItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !imageitem.completed,
            Filter::Selected => imageitem.completed,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filterf {
    All,
    Active,
    Yearset,
}
impl Default for Filterf {
    fn default() -> Self {
        Filterf::All
    }
}
impl Filterf {
    fn matches(&self, file: &File) -> bool {
        match self {
            Filterf::All => true,
            Filterf::Active => !file.yearset,
            Filterf::Yearset => file.yearset,
        }
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}
