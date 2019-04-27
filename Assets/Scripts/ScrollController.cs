using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ScrollController : MonoBehaviour
{
    public float screenDeadzonePercent = 1.0f;
    public GameObject player;

    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        float camWidth = Camera.main.orthographicSize * Screen.width / Screen.height;
        if(Mathf.Abs(player.transform.position.x - transform.position.x) > camWidth * screenDeadzonePercent) {
            transform.position += (Vector3.right * (Mathf.Sign(player.transform.position.x - transform.position.x) *  (Mathf.Abs(player.transform.position.x - transform.position.x) - (camWidth * screenDeadzonePercent))));
        }
    }
}
