(function($) {
	
    "use strict";

    $(document).ready(function() {
		
		/* ----------------------------------------------------------- */
		/*  PRELOADER
		/* ----------------------------------------------------------- */
		
        $("body").toggleClass("loaded");
        setTimeout(function() {
            $("body").addClass("loaded");
        }, 3000);
		
		/* ----------------------------------------------------------- */
		/*  REMOVE # FROM URL
		/* ----------------------------------------------------------- */
		
		$("a[href='#']").on("click", (function(e) {
			e.preventDefault();
		}));
		
		/* ----------------------------------------------------------- */
		/*  TEXT ROTATOR
		/* ----------------------------------------------------------- */
		
		$("#selector").animatedHeadline({
             animationType: "clip"
        });
		
		/* ----------------------------------------------------------- */
		/*  TESTIMONIAL CAROUSEL
		/* ----------------------------------------------------------- */
		
		$("#quote-carousel").carousel({
			wrap: true,
			interval: 13000
		});
		
		/* ----------------------------------------------------------- */
		/*  TESTIMONIAL CAROUSEL TOUCH OPTIMIZED
		/* ----------------------------------------------------------- */ 
		
		var cr = $("#quote-carousel");
		cr.on("touchstart", function(event){
			var xClick = event.originalEvent.touches[0].pageX;
			$(this).one("touchmove", function(event){
				var xMove = event.originalEvent.touches[0].pageX;
				if( Math.floor(xClick - xMove) > 5 ){
					cr.carousel('next');
				}
				else if( Math.floor(xClick - xMove) < -5 ){
					cr.carousel('prev');
				}
			});
			cr.on("touchend", function(){
					$(this).off("touchmove");
			});
		});
		
		/* ----------------------------------------------------------- */
		/*  INITIALIZING MAGNIFIC POPUP
		/* ----------------------------------------------------------- */
		
		jQuery(".magnific-popup-gallery").parent().each(function() {
			magnific_popup_init(jQuery(this))
		});
		var youtubevideo = $('.mfp-youtube');
		if (youtubevideo.length) {		
			jQuery(".mfp-youtube").magnificPopup({
				type: "iframe",
				mainClass: "mfp-fade",
				removalDelay: 0,
				preloader: false,
				fixedContentPos: false,
				iframe: {
					patterns: {
						youtube: {
							src: "https://youtube.com/embed/%id%?autoplay=1&rel=0"
						},
					}
				}
			});
		}
		var vimeovideo = $('.mfp-vimeo');
		if (vimeovideo.length) {	
			jQuery(".mfp-vimeo").magnificPopup({
				type: "iframe",
				mainClass: "mfp-fade",
				removalDelay: 0,
				preloader: false,
				fixedContentPos: false,
				iframe: {
					patterns: {
						vimeo: {
							src: "https://player.vimeo.com/video/%id%?autoplay=1"
						}
					}
				}
			});
		}
		function magnific_popup_init(item) {
			item.magnificPopup({
				delegate: "a[data-gal^='magnific-pop-up']",
				type: "image",
				removalDelay: 500,
				mainClass: "mfp-zoom-in",
				fixedContentPos: false,
				callbacks: {
					beforeOpen: function() {
						// just a hack that adds mfp-anim class to markup 
						this.st.image.markup = this.st.image.markup.replace("mfp-figure", "mfp-figure mfp-with-anim");
					}
				},
				gallery: {
					enabled: true
				}
			});
		}
		
		/* ----------------------------------------------------------- */
		/*  RESUME CARDS ANIMATION
		/* ----------------------------------------------------------- */
		
		if ($(window).width() > 992) {
			$(".resume-list-item").on("click", function() {
				$(".resume-list-item").removeClass("is-active");
				var e = parseInt($(this).data("index"),10);
				$("#resume-list-item-" + e).addClass("is-active");
				var t = e + 1,
					n = e - 1,
					i = e - 2,
					s = e - 3;
				$(".resume-card").removeClass("front back up-front up-up-front back-back"), 
				$(".resume-card-" + e).addClass("front"),
				$(".resume-card-" + t).addClass("back"),
				$(".resume-card-" + n).addClass("back"),
				$(".resume-card-" + i).addClass("back"),
				$(".resume-card-" + s).addClass("back")
			});
		}
		
		/* ----------------------------------------------------------- */
		/*  LOCAL SCROLL
		/* ----------------------------------------------------------- */
		
		$(".scroll-to-target[href^='#']").on("click", function(scroll_to_target) {
			scroll_to_target.preventDefault();
			var a = this.hash,
				i = $(a);
			$("html, body").stop().animate({
				scrollTop: i.offset().top
			}, 900, "swing", function() {})
		})
		
		/* ----------------------------------------------------------- */
		/*  LOGOS SLIDER
		/* ----------------------------------------------------------- */
		
		$("#bxslider").bxSlider({
			minSlides: 1,
			maxSlides: 6,
			slideWidth: 200,
			slideMargin: 30,
			ticker: true,
			speed: 30000
		});	
		
		/* ----------------------------------------------------------- */
		/*  AJAX CONTACT FORM
		/* ----------------------------------------------------------- */
		 
        $(".form-contact").on("submit", function() {
            $(".output_message").text("Loading...");

            var form = $(this);
            $.ajax({
                url: form.attr("action"),
                method: form.attr("method"),
                data: form.serialize(),
                success: function(result) {
                    if (result == "success") {
						$(".form-contact").find(".output_message_holder").addClass("d-block");
						$(".form-contact").find(".output_message").addClass("success");
                        $(".output_message").text("Your message has been sent successfully!");
                    } else {
                        $(".form-contact").find(".output_message_holder").addClass("d-block");
						$(".form-contact").find(".output_message").addClass("error");
                        $(".output_message").text("Error while Sending email! try later");
                    }
                }
            });

            return false;
        });

	});
		
		/* ----------------------------------------------------------- */

})(jQuery);