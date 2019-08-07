//! This program was automatically generated by Visual Embedded Rust. Don't edit here!
#![no_std]                       //  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]        //  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]       //  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(const_transmute)]     //  Allow `transmute` for initialising Mynewt structs
#![feature(proc_macro_hygiene)]  //  Allow Procedural Macros like `run!()`
#![feature(custom_attribute)]    //  Allow Custom Attributes like `#[safe_wrap]`

extern crate cortex_m;                  //  Declare the external library `cortex_m`
extern crate mynewt;                    //  Declare the Mynewt library
extern crate macros as mynewt_macros;   //  Declare the Mynewt Procedural Macros library

use core::panic::PanicInfo; //  Import `PanicInfo` type which is used by `panic()` below
use cortex_m::asm::bkpt;    //  Import cortex_m assembly function to inject breakpoint
use mynewt::{
    result::*,              //  Import Mynewt API Result and Error types
    kernel::os,             //  Import Mynewt OS API
    sys::console,           //  Import Mynewt Console API
    encoding::coap_context::*,  //  Import Mynewt Encoding API
    hw::sensor::{
        self,               //  Import Mynewt Sensor API functions
        sensor_ptr,         //  Import Mynewt Sensor API types
        sensor_arg, sensor_data_ptr, sensor_listener,
        sensor_temp_raw_data, sensor_type_t,
        SensorValue, SensorValueType,
    },
    libs::sensor_network,   //  Import Mynewt Sensor Network Library
    fill_zero, Strn,        //  Import Mynewt macros
};
use mynewt_macros::{ init_strn, strn };  //  Import Mynewt procedural macros


//  let sensor, poll_time, sensor_data, millisec, listener, eventq, SENSOR_DEVICE, payload, SENSOR_POLL_TIME, device_id;

/// Ask Mynewt to poll the temperature sensor every
/// 10 seconds and call `handle_sensor_data()`.
fn start_sensor_listener(sensor, poll_time) -> MynewtResult<()> {
  sensor::set_poll_rate_ms(sensor, poll_time) ? ;
  sensor::register_listener(sensor, "handle_sensor_data") ? ;
  Ok(())
}

/// Compose a CoAP JSON message with the Sensor Key (field name)
/// and Sensor Value in `sensor_data` and send to the CoAP server.
fn send_sensor_data(sensor_data) -> MynewtResult<()> {
  let payload = [["device",device_id].join(),sensor_data].join();
  network::do_server_post() ? ;
  Ok(())
}

/// This listener function is called every 10 seconds by Mynewt
/// to handle the polled sensor data. We convert the sensor
/// data to our transmission format and transmit to the server.
fn handle_sensor_data(sensor_data) -> MynewtResult<()> {
  send_sensor_data(sensor_data) ? ;
  Ok(())
}

/// Describe this function...
fn sensor::set_poll_rate_ms(sensor, millisec) -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn os::sysinit() -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn network::start_server_transport() -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn sensor::register_listener(sensor, listener) -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn os::eventq_run(eventq) -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn network::init_server_post() -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn os::eventq_dflt_get() -> MynewtResult<()> {
  Ok(())
}

/// Describe this function...
fn network::do_server_post() -> MynewtResult<()> {
  Ok(())
}


/// Will be run upon startup to initialise the app
fn on_start() -> MynewtResult<()> {
    let SENSOR_DEVICE = "temp_stm32_0";
    let SENSOR_POLL_TIME = 10000;
    start_sensor_listener(SENSOR_DEVICE, SENSOR_POLL_TIME) ? ;
    network::start_server_transport() ? ;

    //  Return success to `main()`.
    Ok(())
}

/// main() will be called at Mynewt startup. It replaces the C version of the `main()` function.
#[no_mangle]                 //  Don't mangle the name "main"
extern "C" fn main() -> ! {  //  Declare `extern "C"` because it will be called by Mynewt
    //  Initialise Mynewt OS.
    mynewt::sysinit();

    //  Initialise the app.
    on_start()
        .expect("on_start fail");

    //  Start the `forever` task.
    start_task()
        .expect("forever fail");

    //  Mynewt event loop
    loop {                         //  Loop forever...
        os::eventq_run(            //  Process events...
            os::eventq_dflt_get()  //  From default event queue
                .expect("get fail")
        ).expect("run fail")
    }
    //  Never comes here
}

/// This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //  Display the filename and line number in the Semihosting Console.
    console::print("panic ");
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        console::print("at ");       console::buffer(&file);
        console::print(" line 0x");  console::printhex(line as u8);  //  TODO: Print in decimal not hex. Allow more than 255 lines.
        console::print("\n");       console::flush();
    } else {
        console::print("no loc\n");  console::flush();
    }
    //  Pause in the debugger.
    bkpt();
    //  Loop forever so that device won't restart.
    loop {}
}

/*  -- BEGIN BLOCKS --
<xml xmlns="http://www.w3.org/1999/xhtml"><variables><variable type="" id="{0:uoGS8Ut7qbrMxY+DT">sensor</variable><variable type="" id="+}/LZ2zn|5qu,#8oejOB">poll_time</variable><variable type="" id="JrEYqy*^|anEi:~R^~.I">sensor_data</variable><variable type="" id="@k2]dh4RQ4wcej_BOd0w">millisec</variable><variable type="" id="pW#H4#F0sFg9vRWVahxG">listener</variable><variable type="" id="g(}L4e(EcS=?9mr@Sse?">eventq</variable><variable type="" id=",2Y@MNx0?i|V5kdPi{7e">SENSOR_DEVICE</variable><variable type="" id="}BJ%MDhN{/((:45os0[w">payload</variable><variable type="" id="D?Y#zZ*zFk8Jm_p[S5DK">SENSOR_POLL_TIME</variable><variable type="" id="RO2d0yDF8:oHMw[-Uj3t">device_id</variable></variables><block type="on_start" id="^vGc^r(:XZYLALbQLyIo" x="-163" y="-537"><statement name="STMTS"><block type="variables_set" id="i+pd{7.}H{kf=7UzaH7j"><field name="VAR" id=",2Y@MNx0?i|V5kdPi{7e" variabletype="">SENSOR_DEVICE</field><value name="VALUE"><block type="text" id="r}Pwl*3h6;BSjf}6q}_5"><field name="TEXT">temp_stm32_0</field></block></value><next><block type="variables_set" id="GKnT!z1`gQ+xv6#!g80W"><field name="VAR" id="D?Y#zZ*zFk8Jm_p[S5DK" variabletype="">SENSOR_POLL_TIME</field><value name="VALUE"><block type="math_number" id="3rp#U?EVe|SIbr:4|VC$"><field name="NUM">10000</field></block></value><next><block type="procedures_callnoreturn" id="?45T0G%^uW0zb$AF52MS"><mutation name="start_sensor_listener"><arg name="sensor"></arg><arg name="poll_time"></arg></mutation><value name="ARG0"><block type="variables_get" id="/-~CoQ[o^Q~`H118qV7["><field name="VAR" id=",2Y@MNx0?i|V5kdPi{7e" variabletype="">SENSOR_DEVICE</field></block></value><value name="ARG1"><block type="variables_get" id="R3|phL$LS.v]02Xa~T2Z"><field name="VAR" id="D?Y#zZ*zFk8Jm_p[S5DK" variabletype="">SENSOR_POLL_TIME</field></block></value><next><block type="procedures_callnoreturn" id="=L}9h{.d]#2`Ik^C)y(E"><mutation name="network::start_server_transport"></mutation></block></next></block></next></block></next></block></statement></block><block type="procedures_defnoreturn" id="KY`rjgq5Z]H8nKy_%SbR" x="-163" y="-263"><mutation><arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg><arg name="poll_time" varid="+}/LZ2zn|5qu,#8oejOB"></arg></mutation><field name="NAME">start_sensor_listener</field><comment pinned="false" h="80" w="160">Ask Mynewt to poll the temperature sensor every 10 seconds and call `handle_sensor_data()`.</comment><statement name="STACK"><block type="procedures_callnoreturn" id="7zl-|;/*+_em`,/$a^|n"><mutation name="sensor::set_poll_rate_ms"><arg name="sensor"></arg><arg name="millisec"></arg></mutation><value name="ARG0"><block type="variables_get" id="F;QLh/,eA62MaiFP.g1%"><field name="VAR" id="{0:uoGS8Ut7qbrMxY+DT" variabletype="">sensor</field></block></value><value name="ARG1"><block type="variables_get" id=".,}J,XVMp1!~,-?zkobg"><field name="VAR" id="+}/LZ2zn|5qu,#8oejOB" variabletype="">poll_time</field></block></value><next><block type="procedures_callnoreturn" id="jfH_849uIS#u3r0982fN"><mutation name="sensor::register_listener"><arg name="sensor"></arg><arg name="listener"></arg></mutation><value name="ARG0"><block type="variables_get" id="3V_/(y`hbSCQ3REDNFtN"><field name="VAR" id="{0:uoGS8Ut7qbrMxY+DT" variabletype="">sensor</field></block></value><value name="ARG1"><block type="text" id=":R(*i7s;@j/qr9e@:n}A"><field name="TEXT">handle_sensor_data</field></block></value></block></next></block></statement></block><block type="procedures_defnoreturn" id="t9O!Y}._PlhTjJHyAvwn" x="312" y="-263"><mutation><arg name="sensor_data" varid="JrEYqy*^|anEi:~R^~.I"></arg></mutation><field name="NAME">send_sensor_data</field><comment pinned="false" h="80" w="160">Compose a CoAP JSON message with the Sensor Key (field name) and Sensor Value in `sensor_data` and send to the CoAP server.</comment><statement name="STACK"><block type="variables_set" id="Z0$@)|y921DbN0kSddbO"><field name="VAR" id="}BJ%MDhN{/((:45os0[w" variabletype="">payload</field><value name="VALUE"><block type="text_join" id="ukEB%p?m?y%59Ev6g}c:"><mutation items="2"></mutation><value name="ADD0"><block type="text_join" id="cBf4X!9;-TWAc{+fY)j1"><mutation items="2"></mutation><value name="ADD0"><block type="text" id="AsP-SFsjWozHQ~B@Ccm$"><field name="TEXT">device</field></block></value><value name="ADD1"><block type="variables_get" id="S$Po$6mG*:d;/=zn-`}B"><field name="VAR" id="RO2d0yDF8:oHMw[-Uj3t" variabletype="">device_id</field></block></value></block></value><value name="ADD1"><block type="variables_get" id="T[Ju=/CeZ[i;2x(txWn5"><field name="VAR" id="JrEYqy*^|anEi:~R^~.I" variabletype="">sensor_data</field></block></value></block></value><next><block type="procedures_callnoreturn" id="tc/EWZNB{if?Hk[r(w8A"><mutation name="network::do_server_post"></mutation></block></next></block></statement></block><block type="procedures_defnoreturn" id="fl`yW^^_4l+SknDsFvl4" x="-162" y="-37"><mutation><arg name="sensor_data" varid="JrEYqy*^|anEi:~R^~.I"></arg></mutation><field name="NAME">handle_sensor_data</field><comment pinned="false" h="80" w="160">This listener function is called every 10 seconds by Mynewt to handle the polled sensor data. We convert the sensor data to our transmission format and transmit to the server.</comment><statement name="STACK"><block type="procedures_callnoreturn" id="10ko7!S__/Nj7|w(3E?B"><mutation name="send_sensor_data"><arg name="sensor_data"></arg></mutation><value name="ARG0"><block type="variables_get" id="k=r51M^s3H#o.;((~X@y"><field name="VAR" id="JrEYqy*^|anEi:~R^~.I" variabletype="">sensor_data</field></block></value></block></statement></block><block type="procedures_defnoreturn" id="[fJkz[Ay6X2JAYvRINM5" x="37" y="537"><mutation><arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg><arg name="millisec" varid="@k2]dh4RQ4wcej_BOd0w"></arg></mutation><field name="NAME">sensor::set_poll_rate_ms</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defnoreturn" id="Ny4W{:ArnXek8XO}zd[o" x="-263" y="562"><field name="NAME">os::sysinit</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defnoreturn" id="x8HC)hVxiLkuLgW_uU]i" x="462" y="538"><field name="NAME">network::start_server_transport</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defnoreturn" id="`{AbNvOKicSC/jqf@x{x" x="37" y="612"><mutation><arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg><arg name="listener" varid="pW#H4#F0sFg9vRWVahxG"></arg></mutation><field name="NAME">sensor::register_listener</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defnoreturn" id="Xw{:o4uh]%~h6;PKK!yM" x="-262" y="638"><mutation><arg name="eventq" varid="g(}L4e(EcS=?9mr@Sse?"></arg></mutation><field name="NAME">os::eventq_run</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defreturn" id="1(_S?yj!][ZE7)d1b/^:" x="462" y="613"><field name="NAME">network::init_server_post</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defreturn" id="K,gOd#6I?+U_3r}Ki.qh" x="-263" y="712"><field name="NAME">os::eventq_dflt_get</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block><block type="procedures_defnoreturn" id=")Gs9;%U7lg,N1F~dBmh[" x="463" y="712"><field name="NAME">network::do_server_post</field><comment pinned="false" h="80" w="160">Describe this function...</comment></block></xml>
-- END BLOCKS --  */