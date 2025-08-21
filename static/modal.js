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
    let endp = document.getElementById("delete-modal").getAttribute("data-endpoint");

    // Update onclick attribute dynamically
    const deleteButton = document.querySelector("#delete-modal button[type='submit']");
    deleteButton.setAttribute("onclick", `request_delete('${endp}${id}')`);

    // Show modal
    document.getElementById("delete-modal").style.display = "flex";
}

function request_delete(endp){
    fetch(endp, {
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





// -----------------------------------------
//  SEARCH PRODUCT ITEM FUNCTIONALITY 
//  FUNCTIONS UTIILIZED IN BUNDLES PAGE
//
// -------------------------------------------


function addItemModal(element){
    let itemCount = element.getElementsByClassName("item-row").length;
    let endp = element.dataset.endp;
    const newRow = document.createElement('div');
    newRow.className = 'item-row';
    newRow.id = `item-row-${itemCount}`;
    newRow.innerHTML = `
        <div style="position: relative;">
            <input 
                type="text" 
                placeholder="Search Product..."
                hx-get="${endp}"
                hx-target="#item-search-result-${itemCount}"
                hx-swap="innerHtml"
                hx-vals="js:{prefix: this.value}"
                hx-trigger="input changed delay:500ms"
            >
            <input type="hidden" name="list[${itemCount}][id]" required>
            <div id="item-search-result-${itemCount}"
                style="position: absolute; top: 100%; left: 0; width: 100%; z-index: 10; background: white; border: 1px solid #ccc; max-height: 200px; overflow-y: auto;">
            </div>
        </div>
        <input type="number" name="list[${itemCount}][quantity]" required value="" min="0" placeholder="0">
        <span class="close-item" style="cursor: pointer;" onclick="deleteItemModal(this)">&times;</span>
    `;

    element.appendChild(newRow);

    htmx.process(newRow);
    
}

function deleteItemModal(element) {

    let row_item = element.closest('.item-row');
    htmx.remove(row_item)
}
