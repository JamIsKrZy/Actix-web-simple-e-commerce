function limitDecimal(el) {
    // Only allow 2 decimal places
    if (el.value.includes('.')) {
        let [intPart, decPart] = el.value.split('.');
        if (decPart.length > 2) {
        el.value = intPart + '.' + decPart.slice(0, 2);
        }
    }
}

// Get modal element
// var modal = document.getElementById("myModal");

// Open modal
function openModal() {
    document.getElementById("myModal").style.display = "flex";
}

function refresh_table_list(){
    htmx.ajax('GET', '/api/admin/products/list', {target:'#item-list'})
}

// Close modal
function closeModal() {
    let modal = document.getElementById("myModal");
    modal.style.display = "none";

    // Find all inputs, textareas, and selects inside the modal
    const inputs = modal.querySelectorAll("input, textarea, select");

    inputs.forEach(el => {
        if (el.type === "checkbox" || el.type === "radio") {
            el.checked = false; // uncheck checkboxes and radios
        } else {
            el.value = ""; // clear text, number, email, etc.
        }
    });
}


// Close modal when clicking outside
window.onclick = function(event) {
    let modal = document.getElementById("myModal");

    if (event.target === modal) {
        closeModal()
    }
}



document.addEventListener("DOMContentLoaded", () => {
  document.addEventListener("htmx:configRequest", (event) => {
    event.detail.headers['Accept'] = "text/html";
  })
});


