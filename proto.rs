use web_view::*;

fn main() {
    web_view::builder()
    .title("Destincy (Dev) 0.2")
    .size(1280, 685)
    .resizable(false)
    .content(Content::Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
        <link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@700&display=swap" rel="stylesheet">
            <style>
                body {
                    background-image: url('');
                    background-size: cover;
                    position: relative;
                    margin: 0;
                    height: 100vh;
                    display: flex;
                    align-items: flex-end;
                    justify-content: flex-end;
                }
                .box {
                    position: absolute;
                    bottom: 55px;
                    right: 135px;
                    width: 153px;
                    height: 58.5px;
                    background-color: #ffdc2c;
                    border-radius: 29.25px;
                    display: flex;
                    align-items: center;
                    justify-content: flex-start;
                    padding-left: 15px;
                    cursor: default;
                    transition: background-color 0.2s ease;
                }
                
                .circle {
                    width: 33px;
                    height: 33px;
                    background-color: #16171a;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: #ffdc2c;
                    font-size: 16px;
                    transition: background-color 0.2s ease, color 0.2s ease;
                }
                
                .text {
                    font-family: 'Montserrat', sans-serif;
                    font-weight: bold;
                    text-align: center;
                    font-size: 19px;
                    cursor: default;
                    transition: color 0.2s ease;
                    margin-left: auto;
                    margin-right: auto;
                }                

                .box:hover {
                    background-color: #28242c;
                }

                .box:hover .circle {
                    background-color: #ffdc2c;
                    color: #28242c;
                }

                .box:hover .text {
                    color: #ffdc2c;
                }
                .news-tab {
                    position: absolute;
                    top: 0px;
                    left: 50px;
                    width: 435px;
                    height: 435px;
                    overflow: hidden;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                .news-tab img {
                    width: 100%;
                    height: 50%;
                    position: absolute;
                    left: 100%;
                    transition: left 1s;
                }
                .news-tab img.current {
                    left: 0;
                }
                .news-tab .arrow {
                    position: absolute;
                    top: 50%;
                    bottom: 0;
                    width: 30px;
                    background-color: rgba(0, 0, 0, 0.5);
                    color: white;
                    text-align: center;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    opacity: 0;
                    transition: opacity 0.2s;
                    height: 50%;
                }
                .news-tab:hover .arrow {
                    opacity: 1;
                }
                .news-tab .arrow-left {
                    left: 0;
                    top: 50%;
                    z-index: 2;/
                }                
                .news-tab .arrow-right {
                    right: 0;
                    top: 50%;
                }                        
            </style>
            <script>
            window.onload = function() {
                var newsTab = document.querySelector('.news-tab');
                var images = newsTab.querySelectorAll('img');
                var currentIndex = 0;
                var intervalId;
        
                function showImage(index) {
                    for (var i = 0; i < images.length; i++) {
                        images[i].classList.remove('current');
                    }
                    images[index].classList.add('current');
                }
        
                function nextImage() {
                    currentIndex++;
                    if (currentIndex >= images.length) {
                        currentIndex = 0;
                    }
                    showImage(currentIndex);
                }
        
                function prevImage() {
                    currentIndex--;
                    if (currentIndex < 0) {
                        currentIndex = images.length - 1;
                    }
                    showImage(currentIndex);
                }
        
                function resetInterval() {
                    clearInterval(intervalId);
                    intervalId = setInterval(nextImage, 10000);
                }
        
                newsTab.querySelector('.arrow-left').addEventListener('click', function() {
                    prevImage();
                    resetInterval();
                });
        
                newsTab.querySelector('.arrow-right').addEventListener('click', function() {
                    nextImage();
                    resetInterval();
                });
        
                showImage(currentIndex);
                intervalId = setInterval(nextImage, 10000);
            };
        </script>        
            </head>
            <body>
                <div class="news-tab">
                    <div class="arrow arrow-left"><</div>
                    <img src="" class="current">
                    <img src="">
                    <div class="arrow arrow-right">></div>
                </div>
                <div class="box" onclick="window.external.invoke('play')">
                    <div class="circle">▶</div>
                    <div class="text">Play</div>
                </div>
            </body>
            </html>
        "#))
        .user_data(())
        .invoke_handler(|_webview: &mut WebView<'_, ()>, arg| {
            if arg == "play" {
                println!("Play pressed");
            }
            Ok(())
        })
        .build()
        .unwrap()
        .run()
        .unwrap();
    }
