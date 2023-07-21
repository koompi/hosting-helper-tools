pub(crate) fn gen_timer_template() -> String {
    format!(
        r#"[Unit]
Description=Timer for maintenance unifi
    
[Timer]
OnCalendar=*-*-* 00:00:00
Unit=%i.service
    
[Install]
WantedBy=timers.target"#
    )
}

pub(crate) fn gen_service_template() -> String {
    format!(
        r#"[Unit]
Description=Sync Mirror Archlinux Repo for upgrade speedup
        
[Service]
Type=simple
ExecStart=certbot --nginx renew"#
    )
}
