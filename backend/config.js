// OCEAN PRODUCTION BACKEND CONFIGURATION

window.OCEAN_API_BASE_URL =
    "https://ocean-production-2-0.onrender.com";

// Check backend connection
window.checkOceanBackend = async function () {
    try {
        const response = await fetch(
            `${window.OCEAN_API_BASE_URL}/api/health`
        );

        if (!response.ok) throw new Error("Backend error");

        return await response.json();

    } catch (error) {
        console.error("Ocean Backend Error:", error);
        return null;
    }
};

// RUST QUOTATION CALCULATOR
window.calculateOceanQuotation = async function (quotation) {
    try {
        const response = await fetch(
            `${window.OCEAN_API_BASE_URL}/api/quotations/calculate`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(quotation)
            }
        );

        const result = await response.json();

        if (!response.ok) {
            throw new Error(result.error || "Quotation calculation failed");
        }

        return result;

    } catch (error) {
        console.error("Quotation API Error:", error);
        throw error;
    }
};