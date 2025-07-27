mod values;

use iced::widget::{button, column, text, Column, row, container, Button, Row, Text, Container, center, text_input};
use iced::{alignment, Task, Alignment, Length};
use iced::{ Rectangle, Size, Subscription, Theme};
use values::{Commands, Board};
use iced::{Fill, Element, Color,};




struct GameInfo {
    my_board: Board,
    enemy_board: Board,
    started: bool,
    joining: bool,
    loading: String,
    ip_address: String,
}

impl Default for GameInfo {
    fn default() -> GameInfo {
        GameInfo {
            my_board: Board::default(),
            enemy_board: Board::default(),
            started: false,
            joining: false,
            loading: String::new(),
            ip_address: String::new(),
            
        }
    }
}





impl GameInfo {
    fn view(& self) -> Element<Commands, Theme, iced::Renderer> {
        let mut resp;
        if self.started {
            resp = gen_board(&self.my_board);
            
        }else{
            resp = self.start_game();
        }

        Container::new(resp)
            .padding(1)
            .into()

        
        
        
        
    }

    fn update(&mut self, message: Commands) {
        match message {
            Commands::EnteredIP(content) => {
                self.ip_address = content;
            },
            _ => {}
        }
        // ping server with our turn and get update
    }
    
    fn start_game(&self) -> Element<Commands, Theme, iced::Renderer> {
        let mut col = Column::new()
            .push(
                container(
                    Text::new("Welcome to Iced-BattleShip")
                        .size(30)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center))
            )
            .push(
                Text::new("Please enter the IP of the server to connect to")
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center))
            .push(
                text_input("Enter IP here...", &self.ip_address)
                    .on_input(Commands::EnteredIP)
            );
            
        if self.joining {
            col = col
                .push(
                    container(button(Text::new("Joining Server"))).width(Length::Fill).center_x(Length::Fill).padding(20)
                )
                .push(Text::new(&self.loading)
                    .size(20)
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center)
                );
        } else {
            col = col.push(
                container(
                    button(Text::new("Join IP: ")).on_press(Commands::ConnectingToServer(true))
                ).width(Length::Fill).center_x(Length::Fill).padding(20)
            )
        };
        center(col)
            .padding(1)
            .into()
    }
    
}

fn gen_board<'a>(board: &Board)  -> Element<'a, Commands, Theme, iced::Renderer>  {
    let mut board_col = Column::new().spacing(0).align_x(Alignment::Center);
    let mut board_row = Row::new().spacing(0).align_y(Alignment::Center);


    let grid = &board.grid;
    
    for row in grid {
        for cell in row {
            let text = cell.to_string();
            board_row = 
                board_row.push(
                    Button::new(
                        Text::new(text)
                            .width(1080.0 / 10.0)
                            .height(720.0 / 10.0)
                            .align_y(alignment::Vertical::Center)
                    )
                        .padding(0)
                        // select cell here
                        .on_press(Commands::END),
                        // add styling ? 
                );
                    
            
        }
    }
    board_col = board_col.push(board_row);
    board_row = Row::new().spacing(0).align_y(Alignment::Center);
    
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