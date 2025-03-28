# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create SubscriberRequest model struct.`
    -   [x] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [x] Commit: `Implement add function in Notification repository.`
    -   [x] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Commit: `Implement receive_notification function in Notification service.`
    -   [x] Commit: `Implement receive function in Notification controller.`
    -   [x] Commit: `Implement list_messages function in Notification service.`
    -   [x] Commit: `Implement list function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

In this tutorial, we used RwLock<> to synchronise the use of Vec of Notifications. Explain why it is necessary for this case, and explain why we do not use Mutex<> instead?

RwLock (Read-Write Lock) is necessary in this case because we need to handle concurrent access to the Vec of Notifications with different access patterns:
- Multiple readers can access the data simultaneously (when displaying notifications)
- Only one writer can modify the data at a time (when adding new notifications)

Using RwLock instead of Mutex provides better performance in situations with frequent reads and occasional writes, which is the typical pattern for a notification system. While Mutex always blocks all access when locked (even for reading), RwLock allows multiple readers to access the data simultaneously as long as no one is writing, which reduces contention and improves throughput in read-heavy workloads.

In this tutorial, we used lazy_static external library to define Vec and DashMap as a "static" variable. Compared to Java where we can mutate the content of a static variable via a static function, why did not Rust allow us to do so?

Rust doesn't allow direct mutation of static variables because of its core safety principles:

1. Memory safety without garbage collection: Rust achieves this through its ownership system and strict compile-time checks.
2. Thread safety: Mutable static variables would be accessible from any thread without synchronization, leading to potential data races.
3. Immutability by default: Rust prefers immutability to make code safer and easier to reason about.

When we need mutable static variables in Rust, we must use synchronization primitives like RwLock or Mutex to ensure thread safety. The lazy_static library provides a way to initialize static variables at runtime rather than compile-time, allowing for more complex initialization logic while maintaining Rust's safety guarantees.

This is fundamentally different from Java, which allows mutable static variables but relies on the programmer to handle synchronization manually, potentially leading to thread-safety issues if not done correctly.

#### Reflection Subscriber-2

Have you explored things outside of the steps in the tutorial, for example: src/lib.rs? If not, explain why you did not do so. If yes, explain things that you have learned from those other parts of code.

Yes, I did explore the src/lib.rs file to better understand the structure of the application. The lib.rs file contains several important components:
- Configuration management via the APP_CONFIG lazy_static variable which loads environment variables
- HTTP client setup with the REQUEST_CLIENT lazy_static variable
- Error handling utilities with the compose_error_response function
- The Result type alias that's used throughout the application

Understanding these components was crucial for implementing the notification system properly. For instance, the APP_CONFIG provides methods to access configuration values like instance name and URLs, which are essential for creating subscriber requests and handling notifications. The error handling utilities also helped me understand how to properly handle and propagate errors in the Rocket framework context.

Since you have completed the tutorial by now and have tried to test your notification system by spawning multiple instances of Receiver, explain how Observer pattern eases you to plug in more subscribers. How about spawning more than one instance of Main app, will it still be easy enough to add to the system?

The Observer pattern significantly simplifies adding more subscribers to the system. Each Receiver instance (subscriber) can subscribe to specific product types independently, and the Publisher (subject) doesn't need to know the implementation details of each subscriber - it only needs to know their endpoint URLs to notify them.

This decoupling makes it very easy to scale the system by adding more Receiver instances. Each new instance just needs to register itself with the Publisher by making a subscription request, and it will automatically start receiving notifications for the product types it's interested in.

As for spawning multiple Publisher instances (Main apps), it would require additional coordination since subscribers would need to know about and register with each Publisher separately. The current implementation doesn't handle this scenario natively, but it could be extended to support it by:
1. Implementing a registry service that maintains a list of all Publisher instances
2. Having subscribers register with this registry service instead of individual Publishers
3. The registry could then forward subscription requests to all relevant Publishers

Have you tried to make your own Tests, or enhance documentation on your Postman collection? If you have tried those features, tell us whether it is useful for your work (it can be your tutorial work or your Group Project).

Yes, I've enhanced the Postman collection by adding example responses and descriptions to each endpoint. This has been extremely valuable for testing the application and ensuring that all components work together correctly. For instance, I created test scenarios to verify:
- Successfully subscribing to a product type
- Receiving notifications when products of that type are created or updated
- Unsubscribing and confirming that notifications are no longer received

This approach to testing has been invaluable for my group project as well, as it helps us:
1. Document our API behavior clearly for all team members
2. Catch integration issues early in the development process
3. Quickly verify changes without having to manually construct complex HTTP requests
4. Automate some testing processes, saving time and reducing human error

The combination of Postman collections and Rust's strong type system has significantly improved our development workflow by catching many potential issues before they make it to production.
