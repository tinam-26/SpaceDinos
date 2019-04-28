using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.IO;

public class ObjectSpawner : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        SpawnFromJson(System.IO.File.ReadAllText($"{Application.streamingAssetsPath}/levels/1.json"));
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    void SpawnFromJson(string jsonString)
    {
        var level = JsonUtility.FromJson<Level>(jsonString);
        foreach (LevelObject obj in level.levelObjects) {
            var objPrefab = (GameObject)Resources.Load($"objects/{obj.name}", typeof(GameObject));
            Instantiate(objPrefab, new Vector3(obj.x, obj.y, 0), Quaternion.identity);
        }
    }
}
[System.Serializable]
public class LevelObject
{
    public string name;
    public int x;
    public int y;

    public string toString() {
        return $"{name} - ({x}, {y})";
    }
}

[System.Serializable]
public class Level
{    
    public List<LevelObject> levelObjects;

    public string toString() {
        List<string> los = new List<string>();
        foreach(LevelObject obj in levelObjects)
            los.Add(obj.toString());
        return string.Join(", ", los.ToArray());
    }
}