const elements = document.querySelectorAll("[data-observe]");

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if(entry.isIntersecting){
            entry.target.setAttribute("data-visible","true");
        }
    });
});

elements.forEach(el => observer.observe(el));