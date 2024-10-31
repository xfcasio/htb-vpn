use {
    zbus::{ blocking::connection, interface },
    anyhow::Result
};

pub struct AppTray {
     #[allow(unused)] conn: connection::Connection,
}

pub struct TrayIface;

impl AppTray {
    pub fn establish() -> Result<Self> {
        let name = format!("org.kde.StatusNotifierItem-{}-{}", std::process::id(), 0);
        let conn = connection::Builder::session()?
            .name(name.clone())?
            .serve_at("/StatusNotifierItem", TrayIface { })?
            .build()?;
        
        conn.call_method(
            Some("org.kde.StatusNotifierWatcher"),
            "/StatusNotifierWatcher",
            Some("org.kde.StatusNotifierWatcher"),
            "RegisterStatusNotifierItem",
            &name,
        )?;

        Ok(Self { conn })
    }
}

type SniIcon = &'static [(i32, i32, &'static [u8])];

macro_rules! icon {
    () => {
        &[(400, 400, include_bytes!("../htb-icon.argb32"))]
    };
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl TrayIface {
    #[zbus(property)] fn category(&self) -> &'static str { "ApplicationStatus" }
    #[zbus(property)] fn id(&self) -> &'static str { "HTB vpn" }
    #[zbus(property)] fn title(&self) -> &'static str { "HackTheBox network vpn" }
    #[zbus(property)] fn status(&self) -> &'static str { "Active" }
    #[zbus(property)] fn icon_pixmap(&self) -> SniIcon { icon!() }
}


