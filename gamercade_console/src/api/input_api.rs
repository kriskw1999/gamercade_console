use paste::paste;

macro_rules! derive_bind_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse { $($mouse_name:ident,)* },
    ) => {
        paste! {
            pub trait InputApi {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32;
                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<mouse_ $mouse_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mouse_name _released>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mouse_name _held>](&self, player_id: i32) -> i32;
                )*

                fn raw_input_state(&self, player_id: i32) -> i64;
                fn mouse_x(&self, player_id: i32) -> i32;
                fn mouse_y(&self, player_id: i32) -> i32;
                fn raw_mouse_state(&self, player_id: i32) -> i32;
            }

            pub trait InputApiBinding {
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self);
                    fn [<bind_button_ $btn_name _released>](&mut self);
                    fn [<bind_button_ $btn_name _held>](&mut self);
                )*

                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self);
                    fn [<bind_analog_ $anlg_name _y>](&mut self);
                )*

                $(
                    fn [<bind_trigger_ $trg_name>](&mut self);
                )*

                $(
                    fn [<bind_mouse_ $mouse_name _pressed>](&mut self);
                    fn [<bind_mouse_ $mouse_name _released>](&mut self);
                    fn [<bind_mouse_ $mouse_name _held>](&mut self);
                )*

                fn bind_raw_input_state(&mut self);

                fn bind_mouse_x(&mut self);
                fn bind_mouse_y(&mut self);

                fn bind_input_api(&mut self) {
                    $(
                        self.[<bind_button_ $btn_name _pressed>]();
                        self.[<bind_button_ $btn_name _released>]();
                        self.[<bind_button_ $btn_name _held>]();
                    )*

                    $(
                        self.[<bind_analog_ $anlg_name _x>]();
                        self.[<bind_analog_ $anlg_name _y>]();
                    )*

                    $(
                        self.[<bind_trigger_ $trg_name>]();
                    )*

                    $(
                        self.[<bind_mouse_ $mouse_name _pressed>]();
                        self.[<bind_mouse_ $mouse_name _released>]();
                        self.[<bind_mouse_ $mouse_name _held>]();
                    )*

                    self.bind_raw_input_state();
                    self.bind_mouse_x();
                    self.bind_mouse_y();
                }
            }
        }
    };
}

derive_bind_input_api! {
    Buttons {
        a,
        b,
        c,
        d,
        up,
        down,
        left,
        right,
        start,
        select,
        left_shoulder,
        right_shoulder,
        left_stick,
        right_stick,
        left_trigger,
        right_trigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        left,
        right,
        middle,
    },
}
