using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class rotateScript : MonoBehaviour
{
    public GameObject self;
    public float xAngle, yAngle, zAngle; 
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        //rotate on z axis
        self.transform.position = new Vector3(0.75f, 0.0f, 0.0f);
        self.transform.Rotate(xAngle, yAngle, zAngle, Space.Self);
    }
}
