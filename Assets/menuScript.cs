using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class menuScript : MonoBehaviour {
    public uint levelNum;

    // Start is called before the first frame update
    void Start()
    {
        
    }
    
    // Update is called once per frame
    void Update()
    {
        
    }

    void OnMouseDown()
    {
        Debug.Log("clicked");
        ObjectSpawner.level = levelNum;
        SceneManager.LoadScene("MainScene", LoadSceneMode.Single);
    }
   
}
