use serenity::{
    async_trait,
    utils::{
        Colour
    },
    client::{
        Client,
        Context,
        EventHandler
    },
    model::{
        channel::{
            Message
        }
    },
    framework::{
        standard::{
            StandardFramework,
            CommandResult,
            Args,
            macros::{
                command,
                group
            }
        }
    }
};

use chrono::{
    DateTime,
    Local
    };

use std::fs::File;
use std::io::prelude::*;
use std::env;

#[group]
#[commands(
    date,
    time,
    dec_to_bin,
    dec_to_hex,
    hex_to_bin,
    hi,
    learn
    )]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let mut file: File = File::open(".token").unwrap();
    let mut token: String = String::new();
    file.read_to_string(&mut token).expect("Error reading token");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, String::from("Hello ") + &msg.author.name).await?;
    Ok(())
}

#[command]
async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let t: DateTime<Local> = Local::now();
    msg.channel_id.say(&ctx.http, t.format("%I:%M:%p")).await?;
    Ok(())
}

#[command]
async fn date(ctx: &Context, msg: &Message) -> CommandResult {
    let t: DateTime<Local> = Local::now();
    msg.channel_id.say(&ctx.http, t.format("%v")).await?;
    Ok(())
}

#[command]
async fn dec_to_bin(ctx: &Context,msg: &Message, mut args: Args) -> CommandResult {
    match args.single::<u128>() {
       Ok(number) => {
           let bin = format!("{:b}", number);
           msg.channel_id.say(&ctx.http, bin.to_string()).await?;
       }
       Err(_) => {
           msg.channel_id.say(&ctx.http, "The Given value is not a decimal number".to_string()).await?;
       }
    }
    Ok(())
}

#[command]
async fn dec_to_hex(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.single::<u128>() {
        Ok(number) => {
            let hex = format!("{:#X}", number);
            msg.channel_id.say(&ctx.http, hex.to_string()).await?;
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "The Given value is not a decimal number".to_string()).await?;
        }
    }
    Ok(())
}

#[command]
async fn hex_to_bin(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    fn to_bin(c: char) -> &'static str {
        match c {
            '0' => " 0000 ",
            '1' => " 0001 ",
            '2' => " 0010 ",
            '3' => " 0011 ",
            '4' => " 0100 ",
            '5' => " 0101 ",
            '6' => " 0110 ",
            '7' => " 0111 ",
            '8' => " 1000 ",
            '9' => " 1001 ",
            'A' => " 1010 ",
            'B' => " 1011 ",
            'C' => " 1100 ",
            'D' => " 1101 ",
            'E' => " 1110 ",
            'F' => " 1111 ",
            _ => " Invalid hex digit "
        }
    }
    fn convert_to_bin_from_hex(h: &str) -> String {
        h[2..].chars().map(to_bin).collect()
    }
    match args.single::<String>() {
        Ok(hex) => {
            let bin = convert_to_bin_from_hex(&hex);
            msg.channel_id.say(&ctx.http, bin).await?;
        }
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Something went wrong".to_string()).await?;
        }
    }
    Ok(())
}

#[command]
async fn learn(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let pd = String::from(env::current_dir().unwrap().to_string_lossy());
    let cmd = args.single::<String>().unwrap();

    let cmdp = cmd.as_str();
    match cmdp {
       "if_else" => {
           let mut read_syn: File = File::open(pd + "/learn/if_else").unwrap();
           let mut syn = String::new();
           read_syn.read_to_string(&mut syn).unwrap();
           msg.channel_id.send_message(&ctx, |m| {
               m.embed(|e| {
                   e.colour(Colour::from_rgb(0, 235, 120));
                   e.title("Syntax help");
                   e.description("This is the syntax for if/else in Rust language");
                   e.field("syntax", syn, true);
                   e
               });
               m
           }).await?;
       },
       "function" => {
           let mut read_syn: File = File::open(pd + "/learn/function").unwrap();
           let mut syn = String::new();
           read_syn.read_to_string(&mut syn).unwrap();
           msg.channel_id.send_message(&ctx, |m| {
               m.embed(|e| {
                   e.colour(Colour::from_rgb(0, 235, 120));
                   e.title("Syntax help");
                   e.description("This is the syntax for user-defined function in Rust language");
                   e.field("syntax", syn, true);
                   e
               });
               m
           }).await?;
       },
       "match" => {
           let mut read_syn: File = File::open(pd + "/learn/match").unwrap();
           let mut syn = String::new();
           read_syn.read_to_string(&mut syn).unwrap();
           msg.channel_id.send_message(&ctx, |m| {
               m.embed(|e| {
                   e.colour(Colour::from_rgb(0, 235, 120));
                   e.title("Syntax help");
                   e.description("This is the syntax for match in Rust language");
                   e.field("syntax", syn, true);
                   e
               });
               m
           }).await?;
       }
       _ => {
           msg.channel_id.say(&ctx.http, "Keyword dosent exist").await?;
       }
    }
    Ok(())
}
