use crate::native_messaging::{read_message, write_message};
use crate::themes;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Request {
    action: String,
    target: Option<String>,
    size: Option<u32>,
}

/*
           if action == ACTIONS['VERSION']:
           self.send_version()
       elif action == ACTIONS['COLORS']:
           self.send_pywal_colors()
       elif action == ACTIONS['CSS_ENABLE']:
           self.send_enable_css_response(message)
       elif action == ACTIONS['CSS_DISABLE']:
           self.send_disable_css_response(message)
       elif action == ACTIONS['CSS_FONT_SIZE']:
*/

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Response<T> {
    /*
     action: string;
    success: boolean;
    error?: string;
    data?: any;
    */
    action: String,
    success: bool,
    error: Option<String>,
    data: Option<T>, // Error { message: String },
}
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
struct ColorData {
    colors: Vec<String>,
    wallpaper: Option<String>,
}

/*
ACTIONS = {
    'VERSION': 'debug:version',
    'OUTPUT': 'debug:output',
    'COLORS': 'action:colors',
    'INVALID_ACTION': 'action:invalid',
    'CSS_ENABLE': 'css:enable',
    'CSS_DISABLE': 'css:disable',
    'CSS_FONT_SIZE': 'css:font:size',
    'THEME_MODE': 'theme:mode',
}
 */
pub fn run() -> Result<()> {
    // info!("pywalfox-native started asdf");
    // Simple loop: echo ping and log others
    while let Some(msg) = read_message::<Request>()? {
        // info!("> {:?}", msg);

        match msg.action.as_str() {
            "debug:version" => {
                let response = Response {
                    action: "debug:version".to_string(),
                    success: true,
                    error: None,
                    data: Some("2.7.4".to_string()),
                };
                // info!("response {:?}", response);

                let _ = write_message(&response);
            }
            "action:colors" => {
                let colors = themes::read_pywal().expect("Could not read colors from pywal");

                /*


                export interface IPywalData {
                  colors: IPywalColors;
                  wallpaper: string;
                }
                                export interface IPywalColors extends Array<string> {
                  [index: number]: string;
                }


                                 */

                let response = Response {
                    action: "action:colors".to_string(),
                    success: true,
                    error: None,
                    data: Some(ColorData {
                        colors: colors.0,
                        wallpaper: colors.1,
                    }),
                };
                // info!("response {:?}", response);

                let _ = write_message(&response);
                /*
                          (success, pywal_data, message) = get_pywal_colors()
                self.messenger.send_message(Message(
                    ACTIONS['COLORS'],
                    data=pywal_data,
                    success=success,
                    message=message,
                ))
                         */
            }
            _ => {}
        }
    }

    Ok(())
}
