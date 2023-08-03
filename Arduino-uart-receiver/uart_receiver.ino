void setup() {

  Serial.begin(115200);

}

void loop() {
  
  if(Serial.available() > 0) {

    char data = Serial.read(); 

    if(data == 10) {
      data = '\n';
    }

    Serial.print(data);

  }

}
