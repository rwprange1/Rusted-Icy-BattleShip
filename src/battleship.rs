use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use iced::widget::{button, column, text, Column, row, container, Button, Row, Text, Container, center, text_input};
use iced::{alignment, Task, Alignment, Length, color, theme, Subscription, stream};
use iced::{  Size, Theme};
use iced::event::{self, Event};
use iced::{Fill, Element, Color,};
use crate::styles::*;
use crate::client;
use crate::client::ConnectionHandler;

#[derive(Debug, Clone)]
pub enum Message {
    JOIN,
    START,
    LEAVE,
    FIRE,
    EventOccurred,
    BoardClick(Tile)
}

pub struct GameInfo {
    pub my_board: String,
    started: bool,
    joined: bool,
    connection: Option<ConnectionHandler>,
    opponent: String,
    selected_tile: Tile,
    username: String,
}

#[derive(Debug, Clone)]
pub struct Tile{
    pub row: usize,
    pub column: usize,
}

impl Tile{
    pub fn new(row: usize, column: usize) -> Tile{
        Tile{row, column}
    }
}


impl Default for GameInfo {
    fn default() -> GameInfo {
        GameInfo {
            my_board: String::default(),
            started: false,
            joined: false,
            connection: None,
            opponent: String::default(),
            selected_tile: Tile {row: 0, column: 0},
            username: String::default(),
        }
    }
}





impl GameInfo {


    pub fn view(& self) -> Element<Message> {
        if self.joined {
            self.display_game()
        }else {
            self.display_menu()
        }

    }

    pub fn update(&mut self, message: Message) ->Task<Message> {
        match message {
            Message::JOIN => {
                let set = ConnectionHandler::connect();
                self.username = set.username.clone();
                self.connection = Some(set);
                self.joined = true;

                Task::none()

            },

            Message::START => {
                let conn = self.connection.as_mut().unwrap();
                conn.start();
                Task::none()
            },
            Message::LEAVE => {
                Task::none()
            },
            Message::FIRE => {
                let conn = self.connection.as_mut().unwrap();
                conn.fire(self.selected_tile.clone(), self.opponent.clone());

                Task::none()
            },
            Message::EventOccurred => {
                self.joined = true;
                Task::none()
            }
            Message::BoardClick(tile) => {
                println!("{:?}",tile);
                self.selected_tile = tile;
                Task::none()
            }
        }
        
    }

    fn display_menu(&self) -> Element<Message> {


        if self.joined {
            row![text("Please be patient while we connect you to your specified server!").size(30)].into()
        } else {
            row![
                column![
                    text("Welcome to Rust BattleShip!").size(50).center(),
                    row![
                        button(text("Click me to connect and play!").size(30)).on_press(Message::JOIN),
                    ]

                ].padding(20)
            ].into()
        }
    }

    fn display_game(&self) -> Element< Message> {
        let p1_b = gen_board(&[[0;10];10]);
        let p2_b = gen_board(&[[0;10];10]);

        let p1_cont = Container::new(
            column![
                text(self.username.clone()).center(),
               p1_b,
            ]
        ).width(Length::FillPortion(1));

        let p2 = if self.opponent.is_empty() {
            "Player 2".to_string()
        }else{
            self.opponent.clone()
        };

        let p2_cont = Container::new(
            column![
                text(p2.clone()).center(),
               p2_b,
            ]
        ).width(Length::FillPortion(1));

        let header = row![
            text("Rusty BattleShip")
                .size(30)
                .width(Fill)
                .color(ORANGE),
            button("Start Game").on_press(Message::START),
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
    
    


}







/// This function will take a board and return an Element
/// which contains buttons that represent the game state
/// 
/// # Parameters:
/// - board: the current state of the board
/// 
/// # Returns:
/// - the buttons which represent the game state
fn gen_board<'a>(board: &[[u8; 10]; 10])  -> Element<'a, Message, Theme>  {
    let mut board_col = Column::new().spacing(0).align_x(Alignment::Center);
    for i in 0..board[0].len() {
        let mut board_row = Row::new().spacing(0).align_y(Alignment::Center);
        for j in 0..board[0].len()  {
            let square_style: BsBtn = btn_style_water;

            board_row =
                board_row.push(
                    Button::new(
                        Text::new("")
                            .width(20.0)
                            .height(20.0)
                            .align_y(alignment::Vertical::Center)
                    ).style(square_style)
                        .on_press(Message::BoardClick(Tile{row: i, column: j})),

                );
           


        }
        board_col = board_col.push(board_row);
    }

    row![board_col].into()
}






