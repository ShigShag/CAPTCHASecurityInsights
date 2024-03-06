const puppeteer = require('puppeteer');

(async () => {
    // Create browser and page
    const browser = await puppeteer.launch({
        headless: false,
    });

    const page = await browser.newPage();

    // NEW
    // Bypass webdriver condition
    await page.evaluateOnNewDocument(() => {
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined
        });
    });
    // NEW

    // Text if challenge has been solved
    const SolvedIndicator = "I am human";
    await page.goto('https://friendlycaptcha.com/demo');

    // Write name
    await page.type('#name-field', 'John Doe');

    // Wait for finish
    await page.waitForFunction((xpath, desiredContent) => {
        const element = document.evaluate(xpath, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
        return element && element.textContent.trim() === desiredContent;
    }, {}, '/html/body/main/div/form/div/div/div/span', SolvedIndicator);

    // Press button
    await page.click('xpath=/html/body/main/div/form/button');
})();


