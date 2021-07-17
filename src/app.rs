use crate::boxscore::BoxScore;
use crate::pbp::{Play, PlayByPlay};
use crate::utils::{StatefulList, TabTeam, TabsState};

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub boxscore: BoxScore<'a>,
    pub enhanced_graphics: bool,
    pub plays: StatefulList<Play<'a>>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        boxscore: BoxScore<'a>,
        playbyplay: PlayByPlay<'a>,
    ) -> App<'a> {
        App {
            title,
            tabs: TabsState::new(["Game", "Boxscore"]),
            boxscore,
            enhanced_graphics,
            plays: StatefulList::with_items(playbyplay.plays),
        }
    }

    pub fn on_up(&mut self) {
        self.plays.previous();
    }

    pub fn on_down(&mut self) {
        self.plays.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn next_team(&mut self) {
        if self.tabs.index == 1 {
            self.tabs.next_team();
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // TODO: Update progress
    }

    pub fn get_current_team(&self) -> &str {
        match self.tabs.team {
            TabTeam::Home => self.boxscore.h_team.team_id,
            TabTeam::Visitor => self.boxscore.v_team.team_id,
        }
    }
}
