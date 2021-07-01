use maud::{html, Markup, DOCTYPE};

pub fn render_index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Seth Bywater" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";

            link rel="shortcut icon" href="images/favicon.png";

            link rel="stylesheet" href="css/font-awesome.min.css";
            link rel="stylesheet" href="css/bootstrap.min.css";
            link rel="stylesheet" href="css/jquery.animatedheadline.css";
            link rel="stylesheet" href="css/magnific-popup.css";
            link rel="stylesheet" href="css/style.css";
            link rel="stylesheet" href="css/skins/la-palma.css";

            script src="js/modernizr.js"{}
        }
        // Preloader start
        #loader-wrapper {
            #loader {}
            .loader-section.section-left {}
            .loader-section.section-right {}
        }
        // Preloader ends
        // Wrapper starts
        .wrapper {
            // Top section starts
            section.container-fluid.top-section {
                .container {
                    .row {
                        // Right side starts
                        ."col-lg-8 col-md-7 col-sm-8 col-sm-push-4 col-md-push-5 col-lg-push-4 right-side" {
                            // Text rotator starts
                            .selector.uppercase.text-center.text-rotator#selector {
                                h6.text-uppercase { "Hi there! I'm" }
                                h3.ah-headline.text-uppercase {
                                    span.ah-words-wrapper {
                                        b.is_invisible { "Seth Bywater" }
                                        b { "a student" }
                                        b { "a mechatronics engineer" }
                                        b { "a programmer" }
                                    }
                                }
                                // Social media icons
                                .unstyled.hidden-xs {
                                    li { a href="https://github.com/sethbywater"{ i.fa.fa-github{} } }
                                    li { a href="https://www.linkedin.com/in/sethbywater/"{ i.fa.fa-linkedin{} } }
                                }
                            }
                            // Text rotator ends
                        }
                        // Right side ends
                        // Left side starts
                        ."col-lg-4 col-md-5 col-sm-4 col-sm-pull-8 col-md-pull-7 col-lg-pull-8 left-side" {
                            img.img-responsive src="http://via.placeholder.com/540x635" alt="Who's this good looking guy?";
                        }
                        // Left side ends
                    }
                }
            }
            // Top section ends
            // About section starts
            section.about-me#about {
                .container {
                    // Section title starts
                    .row.text-center {
                        .title {
                            h2.center-align.text-uppercase {
                                span { "About " }
                                span { "Me" }
                            }
                            span.title-bg { "Resume" }
                        }
                    }
                    // Section title ends
                    // Resume starts
                    .resume-container {
                        .row {
                            // Resume menu starts
                            .resume-list."col-md-4".hidden-sm.hidden-xs {
                                .resume-list-item.is-active#"resume-list-item-0" data-index="0" {
                                    .resume-list-item-inner.inner-personal-informations {
                                        h6.resume-list-item-title.uppercase { i.fa.fa-vcard{} " My Profile" }
                                    }
                                }
                                .resume-list-item.is-active#"resume-list-item-1" data-index="1" {
                                    .resume-list-item-inner.inner-experience {
                                        h6.resume-list-item-title.uppercase { i.fa.fa-briefcase{} " Experience" }
                                    }
                                }
                                .resume-list-item.is-active#"resume-list-item-2" data-index="2" {
                                    .resume-list-item-inner.inner-education {
                                        h6.resume-list-item-title.uppercase { i.fa.fa-graduation-cap{} " Education" }
                                    }
                                }
                                .resume-list-item.is-active#"resume-list-item-3" data-index="3" {
                                    resume-list-item-inner.inner-skills {
                                        h6.resume-list-item-title.uppercase { i.fa.fa-star{} " Skills" }
                                    }
                                }
                            }
                            // Resume menu ends
                            // Resume content starts
                            ."col-md-8".resume-cards-container."col-sm-12" {
                                .resume-cards {
                                    // Personal information starts
                                    .resume-card."resume-card-0".front data-index="0" {
                                        // Header title
                                        .resume-card-reader {
                                            .resume-card-name { i.fa.fa-vcard{} " My Profile" }
                                        }
                                        // Content
                                        .resume-card-body.personal-informations {
                                            .col-xs-12 p-0 {
                                                p.second-font {
                                                    "I'm currently finishing my degree in Mechatronics Technology at SUNY Delhi. During my time here, I've enjoyed getting my hands dirty designing controls systems and installing them myself. Learning 3D design and simulation has been another highlight."
                                                    br;
                                                    "Programming, mostly in Rust, has been a personal pursuit of mine. Even without a formal education, the spirit of optimization and creative problem solving which drew me to engineering is as present in algorithms as it is in electric motor design."
                                                    br;
                                                    "When I'm not working on projects, I'm either out for a run or working at the cafe."
                                                }
                                                ."col-md-6 col-sm-12 p-0" {
                                                    ul#"second-font list-1 unstyled" {
                                                        li { span { "Name: " } "Seth Bywater" }
                                                        li { span { "Status: " } "Student" }
                                                        li { span { "Languages: " } "English, French" }
                                                        li { span { "Residence: " } "New York, USA" }
                                                        li { span { "Job availability: " } "Open to offers" }
                                                        br;
                                                        li { span { "Hobbies: " } "Running, cooking, and coffee" }
                                                        li { span { "5k personal record: " } "15:37" }
                                                    }
                                                }
                                                ."col-xs-12".buttons {
                                                    a.btn.btn-primary href="/resume" {
                                                        i.fa.fa-file-pdf-o{} "Download my Resume"
                                                    }
                                                    
                                                }
                                            }
                                        }
                                    }
                                    // Personal information ends

                                }
                            }
                            // Resume content ends
                        }
                    }
                }
            }
            // About section ends
        }
        // Wrapper ends
    }
}