use std::env;
use dotenv::dotenv;
use std::sync::Arc;
use tokio_postgres::{NoTls, Error, Client, Connection};


async fn run_query(connection: Arc<Client>, query: &str)-> Result<(), Box<dyn std::error::Error>> {
    connection.execute(query, &[]).await?;
    Ok(())
}

async fn connect_to_postgres(URL: String) -> Result<Arc<Client>, Box<dyn std::error::Error>> {
    let ( client, connection) =
        tokio_postgres::connect(&URL,NoTls).await?;
    Ok(Arc::new(client))
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let string_connection:String = "postgresql://poli:poli453@db:5432/poli".to_string();
    println!("Strin de connect {}", &string_connection);
    let ( client, connection) =
        tokio_postgres::connect(&string_connection,NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let create_table = "
        create table IF NOT EXISTS STUDENTS(
            id serial not null primary KEY,
            first_name VARCHAR(50) not null,
            second_name VARCHAR(50) not null,
            email VARCHAR (255) unique not null
        );
    ";
    let insert_rows = "
        INSERT INTO STUDENTS(first_name,second_name,email)
        VALUES  ('Pedro', 'cohelo', 'coelo12s3@gmail.com'),
            ('Juan', 'cohelo', 'juan1s23@gmail.com');
    ";
    client.execute(create_table,&[]).await?;
    client.execute(insert_rows,&[]).await?;
    println!("Finished");
    Ok(())
}