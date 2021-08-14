🚀 hello-world.rs 🚀
--------------



🚀 Memory safe, blazing fast, minimal and configurable hello world project written in the rust(🚀) programming language  🚀

🚀 While this depends on more c code than rust(🚀) code to compile, because rust(🚀) is magically memory safe, now all c code is memory safe too 🚀

🚀 This project is very minimal, it only requires 1039 crates 🚀

### Installation
To compile this project you need only one library 🚀:

<div>1. The <a href="https://www.cairographics.org/download/">cairo development libraries</a> for cairo-sys 🚀.</div>
<div>1. The <a href="https://www.gtk.org/docs/installations/">libgtk development libraries</a> for pango-sys 🚀.</div>
<div>1. Microsoft/<a href="https://github.com/microsoft/vcpkg">VCPKG</a> for few other c++ libs 🚀.</div>
</br>
<div>Just 1 lib as you can see the number list along the names 🚀</div>
</br>

Then you can just `cargo build --release` and the compiled executable should be located in `./target/release/hello-world`.

Due to the lightweightness of rust, unlike node_modules being fairly large for few dependencies, rust manages compile caches efficiently and stores them to storage to save compile times! Just 33G target folder, the compile time is around 2 hours and 30 minutes on my mac on release mode

![image](https://user-images.githubusercontent.com/57838468/129442903-8dd8ed8a-4afe-43f4-80b3-c082fb72f581.png)

A clean build makes it take around 3.8G
![image](https://user-images.githubusercontent.com/57838468/129435501-01b755d3-1369-4efa-816b-798a5e08790a.png)


### Docker

```sh
$ docker build -t hello-world .
$ docker run -it --rm --name hwrs hello-world
```

### Why rust(🚀) while its only 1 line and depends on 600 c bind crates?

Here are the comments from few of my fellow Rustaceans  🚀

> People ask the question "what's rust(🚀) good for?" pretty frequently, and little terminal apps like this are precisely the reason. [...]. It enables a kind of workflow that simply didn't exist before: I could have a fully safe, "correct", LLVM-optimized binary installed on my desktop in an afternoon.🚀

> Modern rust(🚀) appears pretty similar to modern JavaScript. You declare your variables with let🚀

> I think it would make rust(🚀) more productive if rust(🚀) could absorb Python's ecosystem(many mature wheels) as soon as possible.🚀

> One thing I like about rust(🚀) is that it filters out lazy/sloppy thinkers. Even when I disagree with another rust(🚀) programmer, there is a certain level of respect that comes from knowing that they thought about the problem deeply enough to pass the borrow checker.🚀

> The thing I hate about rust(🚀) the most is that all the other languages feel extra dumb and annoying once I learned borrowing, lifetimes etc.🚀

> "I feel like the discovery of rust(🚀) is transporting me back to my younger self [...]" "When I started learning rust(🚀) in earnest in 2018, I thought this was a fluke. It is just the butterflies you get when you think you fall in love, I told myself."🚀

> rust(🚀)’s product is not a programming language or a compiler. rust(🚀)’s product is the experience of being a rust(🚀) developer🚀

> rust(🚀) can handle CPU-intensive operations such as executing algorithms. 🚀

> Because it’s typically typed, rust(🚀) catches errors at compile time. [...] Also, it compiles code down to machine learning, allowing for extra efficiency.🚀

> Many people try to compare rust(🚀) to Go, but this is flawed. Go is an ancient board game that emphasizes strategy. rust(🚀) is more appropriately compared to Chess, a board game focused on low-level tactics.🚀

> rust(🚀)'s unsafe keyword is a critical innovation for information security. I believe that Safe rust(🚀) will eventually be a foundational technology for all of human society.🚀

> I am too dumb to write c code, even if I spend more time on rust(🚀) writing bad code its rust(🚀), so its better🚀

> Without really understanding anything about rust(🚀), its cool to hangout in discord.gg/rust(🚀) and believe that anything they tell is the right opinion🚀

> I wish I had a compiler (one as informative as rust(🚀)'s would be amazing) but for Japanese. If I could learn Japanese the way I learn programming I'd be conversationally fluent by now.🚀

> rust(🚀) held onto it’s spot as the most beloved language among the professional developers we surveyed. That said, the majority of developers who took the survey aren’t familiar with the language.🚀

> I've experienced modern package management through Cargo and anything below that level now seems like returning to stone age.🚀

> C in "c programming language" stands for cringe 🚀

> Wait its only time until rust(🚀) makes assembly memroy safe.🚀

> Done lots of C/C++/Python is the past, just started learning node/JS recently. Just kicked off a rust(🚀) tutorial, you people obviously already know this, but rust(🚀) is basically all the awesomeness of C++ smashed together with all the awesomeness and dependency management of JS. Looking forward to learning more rust(🚀) in the future! 🚀

> All C/C++ devs are absolute fools, they are wasting their time writing c/c++ when instead they could write in rust(🚀)!!!!

> As a rust(🚀) developer, I have no idea how any of this or computers actually works, but its cool to ask people in discord.gg/rust(🚀) for all help and write code🚀
