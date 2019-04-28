using UnityEngine;
using UnityEngine.SceneManagement;
using System.Collections;
using UnityEngine.UI;

public class PlayerController : MonoBehaviour {
    [HideInInspector] public bool facingRight = true;
    [HideInInspector] public bool jump = false;
    public float moveForce = 365f;
    public float maxSpeed = 5f;
    public float jumpForce = 1000f;
    public float airSpeedModifier = 1.0f;
    public float groundFriction = 0.5f;
    public Transform groundCheckLeft;
    public Transform groundCheckRight;

    private bool grounded = false;
    public Animator anim;
    private Rigidbody2D rb2d;
    public Text coinText;
    public int playerCoin;

    private float prevSpeed = 1.0f;

    // Use this for initialization
    void Awake ()
    {
        //Debug.Log("HEELLOOOOOOOOOO");
        anim = GetComponent<Animator>();
        rb2d = GetComponent<Rigidbody2D>();
        //coinText = GetComponent<Text>();
    }
    
    // Update is called once per frame
    void Update ()
    {
        bool oldGround = grounded;

        grounded = Physics2D.Linecast(transform.position, groundCheckLeft.position, 1 << LayerMask.NameToLayer("Ground")) |
                    Physics2D.Linecast(transform.position, groundCheckRight.position, 1 << LayerMask.NameToLayer("Ground"));

        if(grounded && !oldGround)
            anim.speed = prevSpeed;
        

        if (Input.GetButtonDown("Jump") && grounded)
            jump = true;
        

        if (Input.GetKeyDown(KeyCode.Return)) {
            NextLevel();
        }
    }

    void FixedUpdate()
    {
        float h = Input.GetAxis("Horizontal");

        anim.SetFloat("Speed", Mathf.Abs(h));
        if (grounded) 
            anim.speed = Mathf.Abs(rb2d.velocity.x) / maxSpeed;

        if (Mathf.Abs(rb2d.velocity.x) < 0.1)
            anim.playbackTime = 5.01f;

        if (h * rb2d.velocity.x < maxSpeed)
            rb2d.AddForce(Vector2.right * h * moveForce * (grounded ? 1.0f : airSpeedModifier));

        if (Mathf.Abs (rb2d.velocity.x) > maxSpeed)
            rb2d.velocity = new Vector2(Mathf.Sign (rb2d.velocity.x) * maxSpeed, rb2d.velocity.y);

        if (h > 0 && !facingRight)
            Flip ();
        else if (h < 0 && facingRight)
            Flip ();

        rb2d.sharedMaterial.friction = grounded ? groundFriction : 0.01f;

        if (jump)
        {
            anim.SetTrigger("Jump");
            rb2d.AddForce(new Vector2(0f, jumpForce));

            prevSpeed = anim.speed;
            anim.speed = 0f;

            jump = false;
        }
        
    }

    void Flip()
    {
        facingRight = !facingRight;
        Vector3 theScale = transform.localScale;
        theScale.x *= -1;
        transform.localScale = theScale;
    }

    void NextLevel()
    {
        ObjectSpawner.level++;
        SceneManager.LoadScene(SceneManager.GetActiveScene().buildIndex);
    }

    void OnTriggerEnter2D(Collider2D col)
    {
        if (col.gameObject.CompareTag("coin"))
        {
            col.gameObject.SetActive(false);
            playerCoin++;
            coinText.text = "Coins: " + playerCoin;
        } else if (col.gameObject.CompareTag("spikeyBoi")) {
            SceneManager.LoadScene(SceneManager.GetActiveScene().buildIndex);
        } else if (col.gameObject.CompareTag("Finish")) {
            NextLevel();

        }
    }
}