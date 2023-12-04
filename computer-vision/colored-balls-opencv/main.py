import cv2
import numpy as np

capture = cv2.VideoCapture(0)
capture.set(cv2.CAP_PROP_EXPOSURE, -1)
capture.set(cv2.CAP_PROP_AUTO_EXPOSURE, 1)

cv2.namedWindow("Camera")
cv2.namedWindow("Debug")

lower = (18, 90, 100)
upper = (30, 230, 255)


def on_mouse_click(event, x, y, flags, param):
    if event == cv2.EVENT_LBUTTONDOWN:
        global position
        position = (x, y)
        global new_color
        new_color = True


capture = cv2.VideoCapture(0)
capture.set(cv2.CAP_PROP_EXPOSURE, -1)
capture.set(cv2.CAP_PROP_AUTO_EXPOSURE, 1)

position = (-1, -1)

cv2.setMouseCallback("Camera", on_mouse_click)
new_color = False

while capture.isOpened():
    ret, frame = capture.read()
    frame = cv2.flip(frame, 1)

    hsv = cv2.cvtColor(frame, cv2.COLOR_BGR2HSV)
    hsv = cv2.GaussianBlur(hsv, (11, 11), 0)
    mask = cv2.inRange(hsv, lower, upper)
    mask = cv2.dilate(mask, None, iterations=2)

    contours, _ = cv2.findContours(mask.copy(), cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    if len(contours) > 0:
        contour = max(contours, key=cv2.contourArea)
        (x, y), radius = cv2.minEnclosingCircle(contour)
        M = cv2.moments(contour)
        center = (int(M["m10"] / M["m00"]), int(M["m01"] / M["m00"]))
        cv2.circle(frame, (int(x), int(y)), int(radius), (0, 255, 255), 2)
        cv2.circle(frame, center, 5, (0, 0, 255), -1)

    key = cv2.waitKey(1)
    if key == ord('q'):
        break

    cv2.imshow("Debug", mask)
    cv2.imshow("Camera", frame)

    if new_color:
        bgr = np.uint8([[frame[position[1], position[0]]]])
        hsv = cv2.cvtColor(bgr, cv2.COLOR_BGR2HSV)
        lower = np.array([hsv[0][0][0] - 10, 100, 100])
        upper = np.array([hsv[0][0][0] + 10, 255, 255])

    new_color = False

capture.release()
cv2.destroyAllWindows()
