slint::include_modules!();
use mysql::*;
use mysql::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use slint::{ModelRc,SharedString};
use std::rc::Rc;
use slint::Model;
use slint::VecModel;
#[derive(Default)]
struct AppState {
    customer_ids: Arc<Mutex<ModelRc<SharedString>>>,
    //ids: Vec<String>,
}

fn main() -> Result<(), slint::PlatformError> {
    let url = "mysql://root:password@localhost:3306/northwind";
    let pool = Pool::new(url).unwrap();
    let mut _conn = pool.get_conn().unwrap();
    let mut app_state = AppState::default();
    
    
    let query_result = _conn.query_iter("SELECT id FROM customers").unwrap();

    for row in query_result {
        let row = row.unwrap();
        let customer_id: String = row.get("id").unwrap();

        
        //app_state.customer_ids.lock().unwrap().as_any().downcast_ref::<VecModel<SharedString>>().unwrap().push(SharedString::from(customer_id));
        app_state.customer_ids.lock().unwrap().iter().map(|id: SharedString| id.to_string()).collect::<Vec<_>>().join(",");

    }
//     let customer_ids_shared_strings: Vec<SharedString> = customerIds.iter()
//     .map(|id| SharedString::from(id.clone()))
//     .collect();
// let model_rc_customer_ids = Rc::new(customer_ids_shared_strings);
    let customer_ids = app_state.customer_ids.lock().unwrap().clone();


    // let customer_ids = app_state.customer_ids.lock().unwrap();
    //let _customer_ids_str = customer_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    
    //  let customer_ids_model = ModelRc::<SharedString>::from(customer_ids_str);
        let ui: AppWindow = AppWindow::new()?;
      ui.set_customerIds(customer_ids);

    
    ui.run()
}
