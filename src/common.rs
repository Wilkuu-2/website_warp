use minify_html::Cfg; 
use lazy_static::lazy_static;


lazy_static!{
    pub static ref MINIFY_CONFIG: Cfg = {
        let mut cfg = Cfg::spec_compliant();
        
        cfg.keep_closing_tags=true;

        cfg
    }; 
}


