from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from time import sleep

options = Options()
options.binary_location = "/usr/bin/chromium"
options.add_experimental_option("detach", True)
service = Service(executable_path="/usr/bin/chromedriver")
driver = webdriver.Chrome(service=service, options=options)

driver.execute_cdp_cmd("Page.addScriptToEvaluateOnNewDocument", {
    "source": """
    Object.defineProperty(navigator, 'webdriver', {
     get: () => undefined
    })
  """
})

driver.get('https://captchatest.online/friendly_captcha')

WebDriverWait(driver, 10).until(
    EC.element_to_be_clickable((By.XPATH, '//*[@id="username"]')))
driver.find_element(By.XPATH, '//*[@id="username"]').send_keys('John')

WebDriverWait(driver, 10).until(
    EC.element_to_be_clickable((By.XPATH, '//*[@id="password"]')))
driver.find_element(By.XPATH, '//*[@id="password"]').send_keys('Doe')

sleep(3)

submit_button_xpath = '/html/body/div[1]/main/form/button'
WebDriverWait(driver, 10).until(EC.element_to_be_clickable((By.XPATH, submit_button_xpath)))
driver.find_element(By.XPATH, submit_button_xpath).click()
