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
