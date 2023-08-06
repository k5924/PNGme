<br/>
<p align="center">
  <h3 align="center">PNGme</h3>

  <p align="center">
    A CLI utility to encode messages inside of PNG files
    <br/>
    <br/>
  </p>
</p>



## Table Of Contents

* [About the Project](#about-the-project)
* [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Demonstration](#demonstration)
* [Contributing](#contributing)
* [License](#license)
* [Authors](#authors)
* [Acknowledgements](#acknowledgements)

## About The Project

This is the last project I will build to end off my rust journey. It involves creating a CLI utility that will encode and decode messages from a PNG file. The original tutorial provides unit tests and some example stubbings for the production codes methods but none of the actual business logic for the production code is provided. I was able to implement the majority of the code up to chapter 3 without having to copy from other solutions. From chapter 3 onwards, there is less stubbings/tests and for some reason, a couple of tests would fail due to some logical errors in the original code i used. Because of this, I mainly referred to the implementation by [lijing-2008](https://github.com/lijing-2008/pngme). I still highly recommend tackling this project and seeing how far you get.

## Built With

Rust

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

Ensure rust and cargo is installed on your system. I would recommend installing rust using the rustup utility. If you are on a linux distribution, you may have access to the utility from your package manager or the rust binary through your package manager so I would suggest starting there. 

Link to rust up for other operating systems: https://rustup.rs/

### Installation

1. Clone the repo

```sh
git clone https://github.com/your_username_/Project-Name.git
```

2. Build the project

```sh
cargo build
```

3. Run the project

```sh
cargo run
```

## Demonstration
Check out [this youtube playlist](https://www.youtube.com/playlist?list=PLnvYMKNt9C8hpZapHWdw3pMJX8yzkjmQl) where you can watch the whole build process

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/k5924/PNGme/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.
* Please also read through the [Code Of Conduct](https://github.com/k5924/PNGme/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](https://github.com/k5924/PNGme/blob/main/LICENSE.md) for more information.

## Acknowledgements

* [Picklenerds Tutorial](https://picklenerd.github.io/pngme_book/introduction.html)
* [Picklenerds GitHub](https://github.com/picklenerd)
