let pollInterval = null;
let currentInvoiceId = null;

document.addEventListener("DOMContentLoaded", () => {
  const urlParams = new URLSearchParams(window.location.search);
  currentInvoiceId = urlParams.get("id");

  if (!currentInvoiceId) {
    showError("NO INVOICE_ID SPECIFIED IN PARAMETERS");
    return;
  }

  loadInvoiceDetails(currentInvoiceId);

  document.getElementById("copyUriBtn").addEventListener("click", () => {
    const uri = document.getElementById("checkoutUri").textContent;
    if (uri) {
      navigator.clipboard.writeText(uri).then(() => showToast("[!] ZIP-321 URI COPIED"));
    }
  });

  document.getElementById("confirmPaymentBtn").addEventListener("click", async () => {
    const txidInput = document.getElementById("demoTxid");
    const txid = txidInput.value.trim() || "testnet_tx_" + Math.random().toString(16).substring(2, 12);

    try {
      const res = await fetch(`/api/invoices/${currentInvoiceId}/pay`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ txid }),
      });

      if (!res.ok) throw new Error("Failed to broadcast confirmation");
      const updated = await res.json();
      updateInvoiceUI(updated);
      showToast("[✓] SHIELDED PAYMENT CONFIRMED");
    } catch (err) {
      alert("[ERR] " + err.message);
    }
  });
});

async function loadInvoiceDetails(id) {
  const loadingCard = document.getElementById("loadingCard");
  const checkoutCard = document.getElementById("checkoutCard");

  try {
    const res = await fetch(`/api/invoices/${id}`);
    if (!res.ok) {
      showError("INVOICE NOT FOUND OR EXPIRED");
      return;
    }

    const invoice = await res.json();
    loadingCard.style.display = "none";
    checkoutCard.style.display = "block";

    renderInvoice(invoice);

    if (invoice.status === "pending") {
      startPolling(id);
    }
  } catch (err) {
    showError("CONNECTION FAILED: " + err.message);
  }
}

function renderInvoice(inv) {
  document.getElementById("checkoutAmount").textContent = `${inv.amount} ZEC`;
  document.getElementById("checkoutMemo").textContent = inv.memo ? `MEMO: ${inv.memo}` : "";
  document.getElementById("checkoutUri").textContent = inv.zip321_uri;
  document.getElementById("openWalletBtn").href = inv.zip321_uri;

  const qrContainer = document.getElementById("checkoutQrcode");
  qrContainer.innerHTML = "";

  new QRCode(qrContainer, {
    text: inv.zip321_uri,
    width: 240,
    height: 240,
    colorDark: "#000000",
    colorLight: "#ffffff",
    correctLevel: QRCode.CorrectLevel.M,
  });

  updateInvoiceUI(inv);
}

function updateInvoiceUI(inv) {
  const badgeContainer = document.getElementById("statusBadge");

  if (inv.status === "paid") {
    badgeContainer.innerHTML = `<span class="badge badge-paid">[✓ CONFIRMED_ON_CHAIN]</span>`;
    stopPolling();
  } else {
    badgeContainer.innerHTML = `<span class="badge badge-pending"><span class="pulse-dot"></span> [WAITING_FOR_PAYMENT]</span>`;
  }
}

function startPolling(id) {
  stopPolling();
  pollInterval = setInterval(async () => {
    try {
      const res = await fetch(`/api/invoices/${id}`);
      if (res.ok) {
        const inv = await res.json();
        if (inv.status === "paid") {
          updateInvoiceUI(inv);
          stopPolling();
        }
      }
    } catch (e) {
      console.warn("Polling warning:", e);
    }
  }, 3000);
}

function stopPolling() {
  if (pollInterval) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}

function showError(msg) {
  document.getElementById("loadingCard").style.display = "none";
  document.getElementById("checkoutCard").style.display = "none";
  document.getElementById("errorCard").style.display = "block";
  document.getElementById("errorMsg").textContent = msg;
}

function showToast(msg) {
  const toast = document.getElementById("toast");
  toast.textContent = msg;
  toast.style.display = "block";
  setTimeout(() => {
    toast.style.display = "none";
  }, 2500);
}
