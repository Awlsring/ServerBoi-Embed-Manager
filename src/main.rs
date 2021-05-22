use std::{env};
use a2s::{A2SClient, info::Info};
use chrono::Utc;
extern crate dotenv;

use serenity::{
    http::{
        GuildPagination,
        Http
    },
    model::{
        channel::{
            ChannelType,
            Embed,
            GuildChannel
        },
        id::GuildId
    },
    utils::Color
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Expect env");
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    //Create HTTP session
    let http = Http::new_with_token(&token);

    //Get all guilds bot is in
    let guild_info = http.get_guilds(&GuildPagination::After(GuildId(0)), 100)
        .await
        .unwrap();

    for guild in guild_info {
        println!("Guild: {}", &guild.name);
        
        let channels = http.get_channels(*guild.id.as_u64())
            .await
            .unwrap();

        for channel in channels {
            if channel.kind == ChannelType::Text {
                if channel.name == "serverboi-servers" {
                    println!("Server Channel - Name: {} | ID: {} ", channel.name, channel.id);

                    let messages = channel.messages(&http, |retriever| {
                        retriever.limit(200)
                    })
                        .await
                        .unwrap();

                    for message in messages {
                        let embeds = &message.embeds;

                        for embed in embeds {
                            let address = get_address_from_embed(embed);

                            let server_info = query_server(address);

                            if let Some(info) = server_info {

                                let fields = &embed.fields;

                                let message_id = &message.id;

                                let footer = &embed.footer;

                                let time = Utc::now()
                                    .format("%H:%M");

                                println!("{}", time);

                                channel.edit_message(&http, message_id, |msg| {
                                    msg.embed(|e| {
                                        e.colour(Color::BLURPLE);

                                        e.title(&embed.title
                                            .as_ref()
                                            .unwrap()
                                        );

                                        e.description(&embed.description
                                            .as_ref()
                                            .unwrap()
                                        );

                                        let new_footer = format!("{}", &footer.as_ref().unwrap().text);
                                        let new_footer: Vec<&str> = new_footer
                                            .split('|')
                                            .collect();
                                        let new_footer = format!("{} | {}| 🕒 Pulled at {} UTC", new_footer[0], new_footer[1], time);

                                        e.footer(|f|{
                                            f.text(new_footer);
                                            f.icon_url(
                                                "https://media.giphy.com/media/BuReg1EyvWaac/giphy.gif"
                                            );
                                            f
                                        });

                                        for field in fields {
                                            if field.name == "Players" {
                                                e.field(&field.name, format!("{}/{}", info.players, info.max_players), true);
                                            } else {
                                                e.field(&field.name, &field.value, true);
                                            }
                                        };

                                        e.thumbnail("https://i.kym-cdn.com/entries/icons/original/000/022/255/tumblr_inline_o58r6dmSfe1suaed2_500.gif");
                                        e
                                    })
                                })
                                .await
                                .unwrap();
                            }
                        }
                    }
                }
            }
        };


    }
}

async fn post_embed(channel: &GuildChannel, http: &Http, server_name: String, game: String, players: u8, max_players: u8, ip: &str, port: &str) {
    let _resp = channel.id.send_message(&http, |m| {
        m.embed(|e| {
            e.color(Color::BLURPLE);
            e.title(server_name);
            e.description(format!("Connect: steam://connect/{}:{}", ip, port));
            e.thumbnail("https://i.kym-cdn.com/entries/icons/original/000/022/255/tumblr_inline_o58r6dmSfe1suaed2_500.gif");
            e.field("Status", "Test", true);
            e.field("-", "-", true);
            e.field("Address", format!("`{}:{}`", ip, port), true);
            e.field("Location", "Test", true);
            e.field("Game", format!("{}", game), true);
            e.field("Players", format!("{}/{}", players, max_players), true);
            e.footer(|f| {
                f.text(format!("Owner: {} | 🌎 Hosted on {} in region <region> | 🕒 Pulled at {}", "<owner>", "<provider>", "<time>"));
                f.icon_url("https://media.giphy.com/media/BuReg1EyvWaac/giphy.gif");
                f
            });
            e
        });
        m
    })
    .await
    .unwrap();
}

fn query_server(address: String) -> Option<Info> {
    let client = A2SClient::new()
        .unwrap();

    println!("Getting server info from {}", address);

    for i in 1..11 {
        println!("Attempt {}", i);
        match client.info(&address) {
            Ok(info) => {
                println!("Recieved response. Returning to main.");
                return Some(info)
            },
            Err(_) => println!("No response, retrying"),
        };
    }
    return None
}

fn get_address_from_embed(embed: &Embed) -> String {

    let fields = &embed.fields;

    for field in fields {
        if field.name == "Address" {
            let address = &field.value;
            let address = address.trim_matches('`').to_string();
            println!("IP: {}", address);
            return address
        };
    };

    "".to_string()
}