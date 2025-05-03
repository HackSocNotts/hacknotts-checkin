# HackNotts Checkin

This project is the software that HackNotts uses to print tickets on checkin. When an attendee is checked in, their ticket is automatically printed via a receipt printer.

# Usage

```
Usage: hacknotts-checkin [OPTIONS] --template-path <TEMPLATE_PATH>

Options:
      --template-path <TEMPLATE_PATH>  The path to the ticket's Typst template
      --reference <REFERENCE>          Reference to recheck. For example, JFUD-1
      --account-slug <ACCOUNT_SLUG>    Tito Account slug, required for rechecking
      --account-token <ACCOUNT_TOKEN>  Tito API token, required for rechecking
      --checkin-slug <CHECKIN_SLUG>    Tito Checkin API slug, required for rechecking
      --event-slug <EVENT_SLUG>        Tito Checkin API slug, required for rechecking
  -h, --help                           Print help
  -V, --version                        Print version
  ```

# Setup

**These instructions were written months after HackNotts 24, as an attempt to procrastinate. If you need help, I'll be happy to help. If whoever is responsible for this has disappeared, maybe it's time for you to rewrite this :)**

For HackNotts, this is intended to run on a Raspberry Pi on the front desk, but it can theoretically run on a laptop if required.

## Printing

Somewhere in HackSoc Inventory should be a Star TSP100 thermal printer, which can work on Linux over USB. You can get the CUPS driver [here](https://starmicronics.com/support/products/tsp100iii-support-page/). In CUPS, you'll need to set it to a sensible paper size for your ticket - from what I can tell, the printer doesn't autosize/cut, at least with how this project prints tickets. The ticket template has a width of 72mm, so aim for around there.

This project uses the `lpr` command to print, make sure it's installed.

## Rust

The project is written in Rust (because of course it is). I like https://rustup.rs for setting up Rust. The project can be built as a standard Rust project, with `cargo build --release`. The binary will be in `target/release`.

## Typst, and Customising Tickets

The ticket template is written in [Typst](https://typst.app). Typst is a really nice typesetting system, basically the power of LaTeX with the ease of use of Markdown. The template can be found in `template/main.typ`. To get attendee details in, the following substitutions are made:

- `{{reference}}` - The Tito ticket reference. For example, `HKUW-1`.
- `{{name}}` - The attendee's name. For example, `James Harvey`.
- `{{discord}}` - The attendee's Discord username. For example, `jmshrv`.
- `{{pizza}}` - The attendee's pizza choice. For example, `Pepperoni`.
- `{{release_title}}` - The attendee's release title. For example, `Hacker`.

For more details, check out the documentation in `src/typst_checkin_output.rs`.

Typst will need to be installed separately. This can be done with `cargo install --locked typst-cli`.

## Tito

(This section is especially sparse, I'm just remembering off my head because I can't be bothered to log in)

HackNotts uses [Tito](https://ti.to) to handle ticketing. HackSoc has an account for this, you should be able to find the password on the Vaultwarden. If you haven't made the event yet, I just copied it from the last HackNotts, and I suggest you do the same.

## wh2ws

(Again, off the top of my head. I can help with this for HN25, and update the docs accordingly)

[wh2ws](https://github.com/jmshrv/wh2ws) is a little project I made to bridge a webhook into a websocket. Tito publishes checkins in realtime via a Webhook, but inside eduroam it's much easier to connect to a websocket. Basically, this is what happens:

`TODO: find out what specific hooks Tito needs, and how to setup websockets`

`Tito -> wh2ws.hacksoc.net <-> hacknotts-checkin`

`wh2ws` was set up on the Vaultwarden server, but it died because I forgot to pay for it. It's pretty easy to set up, and I'll get it back into `docker-compose.yml` before I disappear.

With `wh2ws`, you specify Webhook locations, and then give the URL to Tito. `wh2ws` will then let you connect via a Websocket, and any Webhook messages will be immediately passed through.

In the code, this was `wss://wh2ws.hacksoc.net/websocket/checkin-created`, but it should probably be changed to something less guessable.

# Running

Once you've got all of that set up, you should be ready to print tickets! By default, `hacknotts-checkin` runs in a "listen" mode, where it connects to the Websocket and prints off tickets as they come in. For HN24, this worked perfectly, and the Pi was able to sit there for the whole event printing tickets. To run this, you just need to do `hacknotts-checkin --template-path template/main.typ`, assuming your working directory is the root of this codebase.

`hacknotts-checkin` also has a "recheck" mode, where specific tickets can be reprinted if something goes wrong. This also handles checking in the attendee if they aren't already checked in. This needs quite a few extra arguments, which can be seen in the usage section. Again, this wasn't needed for HN24, but you should probably get it working just in case. Rechecks can be run alongside the listen mode - there's no need to stop the listener while you recheck.