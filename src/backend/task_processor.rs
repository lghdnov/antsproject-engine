use std::thread::sleep;
use std::time::Duration;
use deno_core::{JsRuntime, RuntimeOptions};
use deno_core::anyhow::Error;
use deno_core::v8::{Global, Value};

use crate::backend::pool::GameTask;

struct Player{
    name: String,
    code: String,
    runtime: JsRuntime,
}

pub struct TaskProcessor{
    task: GameTask,
    players: Vec<Player>,
}

impl TaskProcessor{
    pub fn new(task: GameTask) -> Self{

        let mut players: Vec<Player> = vec![];

        for bot in &task.bots {

            let runtime = JsRuntime::new(RuntimeOptions {
                ..Default::default()
            });

            players.push(Player{
                name: bot.name.clone(),
                code: bot.code.clone(),
                runtime
            })
        }

        players.reverse();

        TaskProcessor{
            task,
            players
        }
    }

    pub fn initialize_bots(&mut self) -> Result<(), Error> {
        for player in &mut self.players {
            player.runtime.execute_script("bot", player.code.clone())?;

        }
        Ok(())
    }


    pub fn process(&mut self) -> Result<(), Error>{

        for i in 0..10 {
            sleep(Duration::from_secs(2));
            for player in &mut self.players {
                player.runtime.execute_script("updater", include_str!("../javascript/updater.js"))?;
            }
        }

        return Ok(())
    }
}
