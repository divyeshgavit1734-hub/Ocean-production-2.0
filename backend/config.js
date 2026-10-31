// OCEAN PRODUCTION BACKEND CONFIGURATION

window.OCEAN_API_BASE_URL =
    "https://ocean-production-2-0.onrender.com";

window.checkOceanBackend = async function () {
    try {
        const response = await fetch(
            `${window.OCEAN_API_BASE_URL}/api/health`
        );

        if (!response.ok) {
            throw new Error("Backend response error");
        }

        const data = await response.json();

        console.log("Ocean Backend Connected:", data);

        return data;
    } catch (error) {
        console.error("Ocean Backend Connection Failed:", error);
        return null;
    }
};