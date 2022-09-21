pub mod pc {
    pub enum Worlds {
        COBALT,
        CONNERY,
        MILLER,
        JAEGER,
        EMERALD,
        SOLTECH,
        APEX,
        BRIGGS,
    }

    impl super::World for Worlds {
        fn id_u8(&self) -> &'static u8 {
            match self {
                Worlds::COBALT => COBALT_ID,
                Worlds::CONNERY => CONNERY_ID,
                Worlds::MILLER => MILLER_ID,
                Worlds::JAEGER => JAEGER_ID,
                Worlds::EMERALD => EMERALD_ID,
                Worlds::SOLTECH => SOLTECH_ID,
                Worlds::APEX => APEX_ID,
                Worlds::BRIGGS => BRIGGS_ID,
            }
        }
    }

    pub const COBALT_ID: &'static u8 = &13;
    pub const CONNERY_ID: &'static u8 = &1;
    pub const MILLER_ID: &'static u8 = &10;
    pub const JAEGER_ID: &'static u8 = &19;
    pub const EMERALD_ID: &'static u8 = &17;
    pub const SOLTECH_ID: &'static u8 = &40;
    pub const APEX_ID: &'static u8 = &24;
    pub const BRIGGS_ID: &'static u8 = &25;
}

pub trait World {
    fn id_string(&self) -> String {
        return self.id_u8().to_string();
    }
    fn id_u8(&self) -> &'static u8;
}
