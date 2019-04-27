using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class introController : MonoBehaviour
{
    public GameObject thisScene;
    public GameObject nextScene;

    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            Debug.Log("clicked");
            thisScene.SetActive(false);
            nextScene.SetActive(true);
        }
    }

}
