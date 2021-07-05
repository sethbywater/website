function sidebar_open() {
	document.getElementById("sidebar").style.width = "25%";
	document.getElementById("main").style.marginLeft = "25%";
	document.getElementById("sidebar").style.display = "block";
	docoment.getElementById("open-menu").style.display = "none";
}

function sidebar_close() {
	document.getElementById("main").style.marginLeft = "0%";
	document.getElementById("sidebar").style.display = "none";
	document.getElementById("open-menu").style.display = "inline-block";
}

var frame = document.getElementById("frame");
frame.onscroll = function() {
	if frame.pageYOffset < 100 {
		expand_topbar()
	} else {
		collapse_topbar()
	}
}
function expand_topbar() {
	document.getElementById("lil-topbar").style.display = "none";
	document.getElementById("big-topbar").style.display = "block";
}

function collapse_topbar() {
	document.getElementById("big-topbar").style.display = "none";
	document.getElementById("lil-topbar").style.display = "block";
}
