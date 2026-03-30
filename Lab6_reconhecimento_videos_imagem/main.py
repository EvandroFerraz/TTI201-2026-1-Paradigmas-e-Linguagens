import cv2

df = cv2.CascadeClassifier(cv2.data.haarcascades + "haarcascade_frontalface_default.xml")

#camera = cv2.VideoCapture("video.mp4") 
camera = cv2.VideoCapture(0) #usar a camera do computador

while True:
    (sucesso, frame) = camera.read()

    if not sucesso:
        break

    frame_pb = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    faces = df.detectMultiScale(frame_pb, scaleFactor = 1.1, minNeighbors=15, minSize=(30,30), flags=cv2.CASCADE_SCALE_IMAGE)

    # Faz uma cópia do frame
    frame_temp = frame.copy()

    for(x,y,lar,alt) in faces:
        cv2.rectangle(frame_temp, (x,y), (x+lar, y+alt), (0,255,255),2)
        cv2.imshow("Encontrando faces...", frame_temp)
    
    if cv2.waitKey(1) & 0xFF == ord("s"):
        break

camera.release()
cv2.destroyAllWindows()