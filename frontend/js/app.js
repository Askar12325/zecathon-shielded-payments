let qrCodeInstance = null;

document.addEventListener("DOMContentLoaded", () => {
  const form = document.getElementById("createInvoiceForm");
  const refreshBtn = document.getElementById("refreshBtn");
  const copyUriBtn = document.getElementById("copyUriBtn");

  loadInvoices();

  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const amount = parseFloat(document.getElementById("amount").value);
    const memo = document.getElementById("memo").value.trim();
    const createBtn = document.getElementById("createBtn");

    if (isNaN(amount) || amount <= 0) {
      alert("[ERR] Please enter a valid amount greater than 0");
      return;
    }

    createBtn.disabled = true;
    createBtn.textContent = "[PROCESSING REQUEST...]";

    try {
      const res = await fetch("/api/invoices", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ amount, memo: memo || undefined }),
      });

      if (!res.ok) {
        const err = await res.text();
        throw new Error(err || "Failed to create invoice");
      }

      const invoice = await res.json();
      displayInvoiceResult(invoice);
      form.reset();
      loadInvoices();
    } catch (err) {
      alert("[ERR] " + err.message);
    } finally {
      createBtn.disabled = false;
      createBtn.textContent = "[+] GENERATE PAYMENT REQUEST";
    }
  });

  refreshBtn.addEventListener("click", loadInvoices);

  copyUriBtn.addEventListener("click", () => {
    const uri = document.getElementById("resUri").textContent;
    if (uri) {
      navigator.clipboard.writeText(uri).then(() => showToast("[!] ZIP-321 URI COPIED"));
    }
  });
});

function displayInvoiceResult(inv) {
  const card = document.getElementById("invoiceResultCard");
  const amountEl = document.getElementById("resAmount");
  const memoEl = document.getElementById("resMemo");
  const uriEl = document.getElementById("resUri");
  const checkoutBtn = document.getElementById("viewCheckoutBtn");
  const qrContainer = document.getElementById("qrcode");

  amountEl.textContent = `${inv.amount} ZEC`;
  memoEl.textContent = inv.memo ? `MEMO: ${inv.memo}` : "";
  uriEl.textContent = inv.zip321_uri;
  checkoutBtn.href = `/pay.html?id=${inv.invoice_id}`;

  // Clear previous QR code
  qrContainer.innerHTML = "";

  // Render fresh QR code with pure black/white contrast for optimal camera scanning
  qrCodeInstance = new QRCode(qrContainer, {
    text: inv.zip321_uri,
    width: 220,
    height: 220,
    colorDark: "#000000",
    colorLight: "#ffffff",
    correctLevel: QRCode.CorrectLevel.M,
  });

  card.style.display = "block";
  card.scrollIntoView({ behavior: "smooth" });
}

async function loadInvoices() {
  const tbody = document.getElementById("invoicesTableBody");

  try {
    const res = await fetch("/api/invoices");
    if (!res.ok) throw new Error("Connection failed");
    const invoices = await res.json();

    if (invoices.length === 0) {
      tbody.innerHTML = `<tr><td colspan="5" class="empty-state">[NO INVOICES IN CURRENT REGISTRY]</td></tr>`;
      return;
    }

    tbody.innerHTML = invoices
      .map((inv) => {
        const shortId = inv.id.substring(0, 8);
        const statusBadge =
          inv.status === "paid"
            ? `<span class="badge badge-paid">[PAID]</span>`
            : `<span class="badge badge-pending"><span class="pulse-dot"></span> [PENDING]</span>`;

        return `
        <tr>
          <td><code style="color: var(--neon-green);">${shortId}</code></td>
          <td style="font-weight: 700; color: #ffffff;">${inv.amount} ZEC</td>
          <td style="color: var(--text-dim); max-width: 240px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${
            inv.memo || "-"
          }</td>
          <td>${statusBadge}</td>
          <td><a href="/pay.html?id=${inv.id}" class="action-link" target="_blank">[OPEN →]</a></td>
        </tr>
      `;
      })
      .join("");
  } catch (err) {
    tbody.innerHTML = `<tr><td colspan="5" class="empty-state" style="color: var(--danger);">[ERR] ${err.message}</td></tr>`;
  }
}

function showToast(msg) {
  const toast = document.getElementById("toast");
  toast.textContent = msg;
  toast.style.display = "block";
  setTimeout(() => {
    toast.style.display = "none";
  }, 2500);
}
