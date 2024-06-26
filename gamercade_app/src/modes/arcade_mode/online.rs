use eframe::egui::{self, Image};
use gamercade_interface::platform::FrontPageResponse;

use crate::{app::AppDrawContext, local_directory::ImageCache};

use super::{download_window::DownloadWindow, ArcadeActiveView, CreatorDashboardView};

#[derive(Default)]
pub struct OnlineView {
    active_mode: OnlineViewMode,

    pub arcade: ArcadeView,
    pub dashboard: CreatorDashboardView,
    pub download_window: DownloadWindow,
}

#[derive(Default)]
pub struct ArcadeView {
    pub front_page: Option<FrontPageResponse>,
    tab: ArcadeTab,
}

#[derive(Default, PartialEq)]
enum ArcadeTab {
    #[default]
    Popular,
    TopRated,
    New,
}

impl ArcadeView {
    fn draw(&mut self, context: &mut AppDrawContext) {
        let ui = &mut context.ui;

        if let Some(front_page) = &self.front_page {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, ArcadeTab::Popular, "Popular Games");
                ui.selectable_value(&mut self.tab, ArcadeTab::TopRated, "Top Rated");
                ui.selectable_value(&mut self.tab, ArcadeTab::New, "New Releases");
            });

            let slice = match self.tab {
                ArcadeTab::Popular => &front_page.popular_games_ids,
                ArcadeTab::TopRated => &front_page.top_rated_games_ids,
                ArcadeTab::New => &front_page.new_games_ids,
            };

            egui::ScrollArea::vertical().show(context.ui, |ui| {
                egui::Grid::new("arcade_grid")
                    .num_columns(7)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("#");
                        ui.label("Title");
                        ui.label("Short Description");
                        ui.label("Rating");
                        ui.label("Tags");
                        ui.label("Size");
                        ui.label("Download");
                        ui.end_row();

                        for id in slice.iter() {
                            // TOOD: Optimize this
                            if let Some(game) =
                                front_page.games.iter().find(|game| game.game_id == *id)
                            {
                                let image = if let Some(image) =
                                    context.directory.images.games.get(&game.game_id)
                                {
                                    Image::new(image)
                                } else {
                                    Image::new(ImageCache::default_game_image().clone())
                                };
                                ui.add(image.fit_to_exact_size((100.0, 100.0).into()));

                                ui.label(&game.title);
                                ui.label(&game.short_description);
                                ui.label(format!("{}", game.average_rating));
                                ui.label("TODO: Tags");
                                ui.label(format!(
                                    "{}.mb",
                                    game.rom_size.unwrap_or_default() as f32 / (1024.0 * 1024.0)
                                ));

                                // TODO: Replace button with download progress if download in progress
                                // TODO: Disable button if game already downloaded & checksum is accurate
                                // TODO: Make button update if checksum isn't accurate
                                if ui.button("Download").clicked() {
                                    context.task_manager.http.try_download_rom(
                                        game.game_id,
                                        &context.auth_state.get_session().unwrap(),
                                    )
                                }
                            }
                        }
                    });
            });
        } else {
            ui.spinner();
        };
    }
}

#[derive(PartialEq, Default)]
pub enum OnlineViewMode {
    #[default]
    Arcade,
    CreatorDashboard,
}

impl OnlineView {
    pub fn draw(&mut self, ctx: &mut AppDrawContext) -> Option<ArcadeActiveView> {
        ctx.ui.label("Online View");

        self.download_window.draw(ctx);

        ctx.ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_mode, OnlineViewMode::Arcade, "Arcade");
            ui.selectable_value(
                &mut self.active_mode,
                OnlineViewMode::CreatorDashboard,
                "Creator Dashboard",
            );

            ui.separator();

            if ui
                .selectable_label(self.download_window.open, "Downloads")
                .clicked()
            {
                self.download_window.open = !self.download_window.open;
            };
        });

        match self.active_mode {
            OnlineViewMode::Arcade => self.arcade.draw(ctx),
            OnlineViewMode::CreatorDashboard => self.dashboard.draw(ctx),
        }

        if ctx.ui.button("Back").clicked() {
            return Some(ArcadeActiveView::login());
        }

        None
    }
}
