from selenium.webdriver.chrome.webdriver import WebDriver
from selenium.webdriver.common.by import By
from datetime import datetime
import os


def test_wikipedia(driver: WebDriver):
    driver.maximize_window()
    driver.get('https://wikipedia.org/')

    title = driver.find_element(
        By.CSS_SELECTOR, 'span.central-textlogo__image'
    )

    screenshoot_folder = f'screenshots/{datetime.now()}'
    os.makedirs(screenshoot_folder)
    driver.save_screenshot(f'{screenshoot_folder}/wikipedia.png')

    assert title.text == 'Wikipedia'
