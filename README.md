[![Crates.io](https://img.shields.io/crates/v/thirtyfour.svg)](https://crates.io/crates/thirtyfour)
[![Documentation](https://docs.rs/thirtyfour/badge.svg)](https://docs.rs/thirtyfour/)
[![Build Status](https://travis-ci.org/stevepryde/thirtyfour.svg?branch=master)](https://travis-ci.org/stevepryde/thirtyfour)

Thirtyfour is a Selenium WebDriver library for Rust, inspired by the Python Selenium library.

It supports the full W3C WebDriver spec. Tested with Chrome and Firefox although any W3C-compatible WebDriver should work.

Both sync and async APIs are included (see examples below).

## Features

- Async / await support
- Synchronous support
- Create new browser session via Selenium Standalone or Grid
- Automatically close browser session on drop
- All W3C WebDriver and WebElement methods supported
- Find elements (via all common selectors)
- Send keys to elements, including key-combinations
- Execute Javascript
- Action Chains
- Cookies
- Switch to frame/window/element/alert
- Alert support
- Capture / Save screenshot of browser or individual element

## Why 'thirtyfour' ?

It is named after the atomic number for the Selenium chemical element (Se).

## Examples

The examples assume you have a selenium server running at localhost:4444.

The recommended way to do this is via docker, because it automatically takes care of all dependencies, including WebDriver and browser version combinations.

### Setting up Docker and Selenium

To install docker, see [https://docs.docker.com/install/](https://docs.docker.com/install/) (follow the SERVER section if you're on Linux, then look for the Community Edition)

Once you have docker installed, you can start the selenium server, as follows:

    docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-chrome-debug:3.141.59-zinc

We will use the `-debug` container because it starts a VNC server, allowing us to view the browser session in real time.
To connect to the VNC server you will need to use a VNC viewer application. Point it at `localhost:5900` (the default password is `secret`).
If you don't want to use a VNC viewer you don't have to, but then you won't see what is happening when you run the examples.

If you want to run on Firefox instead, or customize the browser setup, see
[docker-selenium](https://github.com/SeleniumHQ/docker-selenium) on GitHub for more options

### Choosing between Sync and Async

The `thirtyfour` library offers both a sync and async API. Which one you should use really depends on your personal preference and the nature of your application.

For a more in-depth introduction to async programming in rust, see [The Rust Async Book](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html).

All interactions with selenium involve sending requests to the selenium server, and then waiting for a response.
The difference between sync and async for `thirtyfour` essentially comes down to what actually happens (or doesn't happen) while waiting for that response.

With the synchronous version, requests to selenium will block the calling thread until they return. While this is obviously inefficient, the code is often a little bit simpler and easier to follow.
You can alleviate some of the inefficiency by using multiple threads if/when concurrency is required, but note that threads have some overhead as well. If you're thinking of spawning hundreds (or even tens) of threads, you probably want to use async instead.

In contrast, async method calls will immediately return a "future", which is a bit like a task that can be executed later (typically via `.await`), and will eventually complete and return a value.
While it is `.await`ing a response from selenium, the "future" will yield control of the current thread, allowing other "futures" to run.
This allows multiple operations to be performed concurrently on the same thread, rather than just sitting and blocking the thread completely for each call to selenium.
For more information on running multiple tasks concurrently, see chapter 6 of [The Rust Async Book](https://rust-lang.github.io/async-book/06_multiple_futures/01_chapter.html).

In short, if you need to do a lot of I/O (network requests, reading/writing files, handling network connections), and especially if you want to do I/O operations concurrently, choose async. If not, it's really up to your personal preference.

### Async example:

To run this example:

    cargo run --example async

NOTE: This example does not make use of concurrency, as it is only a demonstration of how to write a simple test using `async/.await` syntax.

```rust
use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;

     // Navigate to https://wikipedia.org.
     driver.get("https://wikipedia.org").await?;
     let elem_form = driver.find_element(By::Id("search-form")).await?;

     // Find element from element.
     let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

     // Type in the search terms.
     elem_text.send_keys("selenium").await?;

     // Click the search button.
     let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await?;
     elem_button.click().await?;

     // Look for header to implicitly wait for the page to load.
     driver.find_element(By::ClassName("firstHeading")).await?;
     assert_eq!(driver.title().await?, "Selenium - Wikipedia");

     Ok(())
}
```

### Sync example:

To run this example:

    cargo run --example sync

```rust
use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, DesiredCapabilities, sync::WebDriver};

fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)?;

     // Navigate to https://wikipedia.org.
     driver.get("https://wikipedia.org")?;
     let elem_form = driver.find_element(By::Id("search-form"))?;

     // Find element from element.
     let elem_text = elem_form.find_element(By::Id("searchInput"))?;

     // Type in the search terms.
     elem_text.send_keys("selenium")?;

     // Click the search button.
     let elem_button = elem_form.find_element(By::Css("button[type='submit']"))?;
     elem_button.click()?;

     // Look for header to implicitly wait for the page to load.
     driver.find_element(By::ClassName("firstHeading"))?;
     assert_eq!(driver.title()?, "Selenium - Wikipedia");

     Ok(())
}
```

## Running the tests for `thirtyfour`, including doctests

You generally only need to run the tests if you plan on contributing to the development of `thirtyfour`. If you just want to use the crate in your own project, you can skip this section.

Just like the examples above, the tests in this crate require a running instance of Selenium at `http://localhost:4444`.

If you still have the docker container running from the examples above, you will need to bring it down (otherwise the next step will complain that port 4444 is already in use):

    docker stop selenium-server 

The tests also require a small web app called `webappdemo` that was purpose-built for testing the `thirtyfour` crate.

Both can be run easily using docker-compose. To install docker-compose, see [https://docs.docker.com/compose/install/](https://docs.docker.com/compose/install/)

Once you have docker-compose installed, you can start the required containers, as follows:

    docker-compose up -d

Then, to run the tests:

    cargo test -- --test-threads=1

We need to limit the tests to a single thread because the selenium server only supports 1 browser instance at a time.
(You can increase this limit in the `docker-compose.yml` file if you want. Remember to restart the containers afterwards)

If you need to restart the docker containers:

    docker-compose restart 

And finally, to remove them:

    docker-compose down

## LICENSE

This work is dual-licensed under MIT or Apache 2.0.
You can choose either license if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
