use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::std_msgs,pr_info,
};

fn main() -> Result<(), DynError>{
    let ctx = Context::new()?;
    let node = ctx.create_node("run_motor", None, Default::default())?;
    let publisher = node.create_publisher::<drobo_interfaces::msg::MdLibMsg>("md_driver_topic", None)?;
    let mut pub_msg = drobo_interfaces::msg::MdLibMsg::new().unwrap();
    pub_msg.address = 0x00;
    pub_msg.mode = 2;

    let logger = Logger::new("run_motor");

    let mut selelctor = ctx.create_selector()?;


    loop {
        let phase = true;
        let power = 500;
        pub_msg.phase = phase;
        pub_msg.power = power;
        pr_info!(logger, "中央輪, phase: {}, power: {}", phase, power);
        publisher.send(&pub_msg).unwrap();
    }
}