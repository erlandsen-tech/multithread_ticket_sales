use std::thread;
extern crate num_cpus;
extern crate rand;
use rand::Rng;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct BoxOffice {
    tickets: Vec<Ticket>,
    account: i32,
    name: String,
}
#[derive(Clone)]
struct Ticket {
    price: i32,
    number: i32,
    show: String,
}

fn ticket_sales(mut box_office: &mut BoxOffice, ticket_id: usize, amount: i32) {
    let mut ticket = &mut box_office.tickets[ticket_id];
    if (ticket.number - amount) > 0 {
        box_office.account = box_office.account + (ticket.price * amount) as i32;
        ticket.number = ticket.number - amount;
    }
    else{
        box_office.account = box_office.account + (ticket.price * ticket.number) as i32;
        ticket.number = 0;
    }
}
fn box_office_status(box_office: &BoxOffice) -> i32 {
    let mut tickets: i32 = 0;
    let len = box_office.tickets.len();
    if len > 0 {
        for i in 0..len {
            tickets = tickets + box_office.tickets[i].number as i32;
            println!(
                "There are {} tickets for \"{}\"",
                box_office.tickets[i].number, box_office.tickets[i].show
            );
        }
    }
    println!(
        "{} has {} tickets for sale and {} kr in account",
        box_office.name, tickets, box_office.account
    );
    if tickets > 0 {
        tickets
    } else {
        0
    }
}
fn multi_buy(mut box_office: BoxOffice) -> BoxOffice
{
        let (tx, rx) = channel();
        let data = Arc::new(Mutex::new(box_office.clone()));

        let number_of_physical_cores = num_cpus::get();
        let mut children = vec![];
        for _i in 0..number_of_physical_cores {
            let (data, tx) = (Arc::clone(&data), tx.clone());

            children.push(thread::spawn(move || {
                let amount = rand::thread_rng().gen_range(1, 20);
                let mut data = data.lock().unwrap();
                ticket_sales(data.deref_mut(), 0, amount);
                tx.send(data.deref().clone()).expect("Something went bad");
            }));
        }
        for child in children {
            let _ = child.join();
        }

        box_office = rx.recv().unwrap();
        box_office
}
fn main() {
    let show = String::from("Linux on ice");
    let ticket = Ticket {
        price: 180,
        number: 100,
        show,
    };

    let mut box_office = BoxOffice {
        tickets: vec![ticket],
        account: 0,
        name: String::from("Edda Kino"),
    };

    loop {
        box_office = multi_buy(box_office);
        if box_office_status(&box_office) <= 0 {
            break;
        }
    }
}
