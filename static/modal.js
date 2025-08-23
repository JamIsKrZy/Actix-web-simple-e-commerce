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
                hx-trigger="input changed delay:500ms, focus"
            >
            <input class="hide-input" id="product-id" type="number" name="items[${itemCount}][product_id]" required>
            <div id="item-search-result-${itemCount}" class="result-options"></div>
        </div>
        <input type="number" name="items[${itemCount}][quantity]" value="" min="0" placeholder="0" required>
        <span class="close-item" style="cursor: pointer;" onclick="deleteItemModal(this)">&times;</span>
    `;

    element.appendChild(newRow);

    htmx.process(newRow);
    
}

function deleteItemModal(element) {

    let row_item = element.closest('.item-row');
    htmx.remove(row_item)
}


// BUNDLE PAGE FUNCTIONALITY
// 
// Action listener if modal item 
// 



document.addEventListener("DOMContentLoaded", () => {
  // Show dropdown after htmx swaps results in
    document.body.addEventListener("htmx:afterSwap", (e) => {
    const target = e.detail.target;
        if (target.classList.contains("result-options")) {
            // Hide all other dropdowns first
            document.querySelectorAll(".result-options").forEach(d => {
            if (d !== target) d.style.display = "none";
            });

            // Show if it has <li> children
            if (target.querySelector("li")) {
            target.style.display = "block";
            } else {
            target.style.display = "none";
            }
        }
    });

    document.body.addEventListener("input", (e) => {
        if (e.target.matches(".item-row input[type='text']")) {
            const container = e.target.closest("div");
            const hiddenInput = container.querySelector("input[type='hidden']");
            if (hiddenInput) {
                hiddenInput.value = ""; // clear hidden value on manual change
            }
        }
    }); 

  // Handle clicking on an <li> item
  document.body.addEventListener("click", (e) => {
    if (e.target.tagName === "LI" && e.target.closest(".result-options")) {
        const dropdown = e.target.closest(".result-options");
        const container = dropdown.closest("div").parentElement; // the relative container

        const textInput = container.querySelector("input[type='text']");
        const hiddenInput = container.querySelector("#product-id");

        if (textInput && hiddenInput) {
            textInput.value = e.target.dataset.name || "";
            hiddenInput.value = parseInt(e.target.dataset.id, 10) | "";   
        }
        
        dropdown.style.display = "none";
    } 
    else if (!e.target.closest(".item-row")) {
        document.querySelectorAll(".result-options").forEach(d => {
        d.style.display = "none";
        });
    }
    });
});

