mod values;
mod utils;

use iced::widget::{button, column, text, Column, row, container, Button, Row, Text, Container, center, text_input};
use iced::{alignment, Task, Alignment, Length, color, theme};
use iced::{ Rectangle, Size, Subscription, Theme};
use values::{Commands, Board};
use iced::{Fill, Element, Color,};
use iced::keyboard::key::Named::MediaApps;
use utils::*;



impl Default for GameInfo {
    fn default() -> GameInfo {
        GameInfo {
            my_board: Board::default(),
        }
    }
}





impl GameInfo {



    
    fn view(& self) -> Element<Message> {
        let p1_b = gen_board(&self.my_board);
        let p2_b = gen_board(&self.my_board);
        
        let p1_cont = Container::new(
            column![
                text("Player 1").center(),
               p1_b,
            ]
        ).width(Length::FillPortion(1));

        let p2_cont = Container::new(
            column![
                text("Player 2").center(),
               p2_b,
            ]
        ).width(Length::FillPortion(1));
        
        let header = row![
            text("Rusty BattleShip")
                .size(30)
                .width(Fill)
                .color(ORANGE),
            button(text("Confirm Shot").color(GREEN).size(20)).on_press(Message::FIRE),
            button(text("Quit").color(RED).size(20)).on_press(Message::LEAVE),
        ].spacing(8);


        container(
            column![
                header,
                row![p1_cont,p2_cont].width(Length::Fill).spacing(10)
            ].spacing(10), // Add spacing between header and columns
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
        
        

    }

    fn update(&mut self, message: Message) {

    }
    

    
}

fn gen_board<'a>(board: &Board)  -> Element<'a, Message, Theme>  {
    let mut board_col = Column::new().spacing(0).align_x(Alignment::Center);



    let grid = &board.grid;
    let mut cnt = 0;
    for row in grid {
        cnt = 0;
        let mut board_row = Row::new().spacing(0).align_y(Alignment::Center);
        for _cell in row {
            let square_style: BsBtn = btn_style_water;

            board_row = 
                board_row.push(
                    Button::new(
                        Text::new("")
                            .width(20.0)
                            .height(20.0)
                            .align_y(alignment::Vertical::Center)
                    ).style(square_style)
                        .on_press(Message::LEAVE),

                );
            cnt +=1;
                    
            
        }
        board_col = board_col.push(board_row);
    }

    row![board_col].into()
}








fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: Size {
            width: 1080.0,
            height: 720.0,
        },
        resizable: true,
        exit_on_close_request: true,
        ..iced::window::Settings::default()
    };
    
    iced::application("Online Battle Ship", GameInfo::update, GameInfo::view)
        .window(window_settings)
        .run()
    

}