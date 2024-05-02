const puppeteer = require('puppeteer');

(async () => {
    const browser = await puppeteer.launch({
        headless: true,
    });
    const page = await browser.newPage();

    await page.evaluateOnNewDocument(() => {
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined
        });
    });

    await page.goto('https://captchatest.online/turnstile');
    await page.type('xpath=//*[@id="username"]', 'John');
    await page.type('xpath=//*[@id="password"]', 'Doe');

    // Click on the checkbox
    await new Promise(resolve => setTimeout(resolve, 3000));

    // Listen for all responses after the click
    page.on('response', async (response) => {
        const request = response.request();
        if (request.method() === 'POST') {
            console.log(`Received response for ${request.url()}`);
            if (response.status() === 200) {
                console.log('Success: POST request was successful');
            } else if (response.status() >= 400) {
                console.log(`Error: POST request failed with status ${response.status()}`);
            }
        }
    });

    await page.click('xpath=/html/body/div[1]/main/form/button');

    await browser.close()
})();


