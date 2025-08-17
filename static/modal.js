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
// var modal = document.getElementById("create-model");

// Open modal
function openCreateModal() {
    document.getElementById("create-modal").style.display = "flex";
}


function openDeleteModal(button) {
    const row = button.closest("tr");
    const id = row.dataset.id;

    const name = row.getElementsByTagName('td')[0].textContent;


    // Update modal text
    document.getElementById("delete-who").textContent = name;

    // Update onclick attribute dynamically
    const deleteButton = document.querySelector("#delete-modal button[type='submit']");
    deleteButton.setAttribute("onclick", `request_delete('products', ${id})`);

    // Show modal
    document.getElementById("delete-modal").style.display = "flex";
}

function request_delete(what, id){
    fetch(`/api/admin/${what}/${id}`, {
        method: 'DELETE', // Specify DELETE
        headers: {
            'Content-Type': 'application/json', // Optional: depends on backend
        },
    })
    .then((response) => {
        if (response.ok){
            refresh_table_list();
        } else {
            console.error("Failed to delete!")
        }

        closeDeleteModal();
    })
}


function refresh_table_list(){
    htmx.ajax('GET', '/api/admin/products/list', {target:'#item-list'})
}

function closeModal(modal) {
    modal.style.display = "none";
}

// Close Delete
function closeDeleteModal() {
    let modal = document.getElementById("delete-modal");
    closeModal(modal);
}

// Close create modal
function closeCreateModal() {
    let modal = document.getElementById("create-modal");
    closeModal(modal);

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
    let create_modal = document.getElementById("create-model");
    let delete_modal = document.getElementById("delete-model");

    if (event.target === create_modal) {
        closeCreateModal()
    } else if (event.target === delete_modal){

    }
}



document.addEventListener("DOMContentLoaded", () => {
  document.addEventListener("htmx:configRequest", (event) => {
    event.detail.headers['Accept'] = "text/html";
  })
});


