import zmq
import cv2
import numpy as np
from math import pi


def find_contours(frame):
    luv = cv2.cvtColor(frame, cv2.COLOR_BGR2LUV)
    _, _, v = cv2.split(luv)
    blured = cv2.medianBlur(v, 15)
    canny = cv2.Canny(blured, 30, 10)
    dilated = cv2.dilate(canny, None, iterations=1)
    contours, _ = cv2.findContours(dilated, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    return contours


def count_figures(frame, contours, min_area: int = 150):
    figures = {
        'circles': 0,
        'rects': 0,
    }

    contours = filter(lambda c: cv2.contourArea(c) > min_area, contours)

    for contour in contours:
        rect = cv2.minAreaRect(contour)
        box = cv2.boxPoints(rect)
        box = np.intp(box)

        (x, y), rad = cv2.minEnclosingCircle(contour)
        center = int(x), int(y)
        rad = int(rad)
        area_rect = cv2.contourArea(box)
        area_circle = pi * rad * rad

        if area_circle < area_rect:
            cv2.circle(frame, center, rad, (255, 255, 255), 2)
            figures['circles'] += 1
        else:
            cv2.drawContours(frame, [box], 0, (255, 255, 255), 2)
            figures['rects'] += 1

    return figures


def process_image(frame):
    contours = find_contours(frame)
    figures = count_figures(frame, contours)
    circles, rects = figures['circles'], figures['rects']

    cv2.putText(frame, f'{circles=} {rects=}', (10, 30), cv2.FONT_HERSHEY_SIMPLEX, 1, (255, 255, 255), 2)
    cv2.imshow('Camera', frame)

    return circles, rects


def find_objects(protocol: str, ip4: str, port: int):
    context = zmq.Context()
    socket = context.socket(zmq.SUB)
    socket.setsockopt(zmq.SUBSCRIBE, b"")
    socket.connect(f'{protocol}://{ip4}:{port}')

    while True:
        buffer = socket.recv()
        arr = np.frombuffer(buffer, np.uint8)
        frame = cv2.imdecode(arr, -1)
        circles, rects = process_image(frame)
        print(f'{circles=}, {rects=}')

        if cv2.waitKey(1) & 0xFF == ord('q'):
            print('Camera stopped by keyboard')
            break


def main():
    cv2.namedWindow('Camera')

    find_objects('tcp', '192.168.0.105', 6556)

    cv2.destroyAllWindows()


if __name__ == '__main__':
    main()