using System.Collections.Generic;
using System.Drawing;
using System.Threading;

class ColourPrinter {
    public static string Reset = "\x1b[39m";

    public static string getColourCode(int r, int g, int b) {
        return $"\x1b[38;2;{r};{g};{b}m";
    }

    public static string getColorString(int r, int g, int b) {
        return $"{getColourCode(r,g,b)}█{Reset}";
    }
    public static void Print(int r, int g, int b) {
        string toPrint = $"{getColourCode(r,g,b)}█{Reset}";
        //Console.WriteLine(toPrint);
        Console.Write(toPrint);
    }
}

class ImageProcess {
    public Bitmap Img { get; set; }
    public int Height { get; set; }
    public int Width { get; set; }

    public ImageProcess(string filepath) {
        var tempImg = new Bitmap(filepath);

        var hpercent = (float)Console.WindowHeight / (float)tempImg.Height;
        var wsize = (tempImg.Width * hpercent) * 2;

        Img = new Bitmap(tempImg, new Size((int)wsize, Console.WindowHeight - 2));
        Height = Img.Height;
        Width = Img.Width;
    }

    public (int, int , int) getPixelRGB(int x, int y) {
        Color pixel = Img.GetPixel(x, y);
        return (pixel.R, pixel.G, pixel.B);
    }

    public List<List<string>> toArray() {
        List<List<string>> imageArray = new List<List<string>>();
        for (int y = 0; y < Height; y++) {
            List<string> empty_array = new List<string>();
            imageArray.Add(empty_array);
        }

        for (int y = Height - 1; y >= 0; y--) {
            for (int x = 0; x < Width; x++) {
                (int r, int g, int b) = getPixelRGB(x, y);
                
                imageArray[y].Add(ColourPrinter.getColorString(r, g, b));
            }
        }

        return imageArray;
    }

    public void printToTerminal() {
        Console.Clear();

        var ArrayImage = toArray();
        var spacesForCenter = (Console.WindowWidth / 2) - (ArrayImage[0].Count / 2);
        var centerGap = new string(' ', spacesForCenter);

        var imageString = centerGap;

        foreach (List<string> row in ArrayImage) {
            foreach (string pixel in row) {
                imageString += pixel;
            }
            imageString += $"\n{centerGap}";
        }

        Console.Write(imageString);
    }
}

class GifProcess {
    public Bitmap Img { get; set; }
    public int Height { get; set; }
    public int Width { get; set; }

    public GifProcess(Image tempImg) {
        var hpercent = (float)Console.WindowHeight / (float)tempImg.Height;
        var wsize = (tempImg.Width * hpercent) * 2;

        Img = new Bitmap(tempImg, new Size((int)wsize, Console.WindowHeight - 2));
        Height = Img.Height;
        Width = Img.Width;
    }

    public (int, int , int) getPixelRGB(int x, int y) {
        Color pixel = Img.GetPixel(x, y);
        return (pixel.R, pixel.G, pixel.B);
    }

    public List<List<string>> toArray() {
        List<List<string>> imageArray = new List<List<string>>();
        for (int y = 0; y < Height; y++) {
            List<string> empty_array = new List<string>();
            imageArray.Add(empty_array);
        }

        for (int y = Height - 1; y >= 0; y--) {
            for (int x = 0; x < Width; x++) {
                (int r, int g, int b) = getPixelRGB(x, y);
                
                imageArray[y].Add(ColourPrinter.getColorString(r, g, b));
            }
        }

        return imageArray;
    }

    public string completeImageString() {
        Console.Clear();

        var ArrayImage = toArray();
        var spacesForCenter = (Console.WindowWidth / 2) - (ArrayImage[0].Count / 2);
        var centerGap = new string(' ', spacesForCenter);

        var imageString = centerGap;

        foreach (List<string> row in ArrayImage) {
            foreach (string pixel in row) {
                imageString += pixel;
            }
            imageString += $"\n{centerGap}";
        }

        return imageString;
    }

    public static void printToTerminal(Image[] gif_frames) {
        var complete_gif_string = new List<string>();

        foreach (Image img in gif_frames) {
            var baseImage = new GifProcess(img);
            complete_gif_string.Add(baseImage.completeImageString());
        }
        foreach (string gif_frame in complete_gif_string) {
            Console.Clear();
            Console.Write(gif_frame);
            Thread.Sleep(100);
        }
    }

    public static Image[] GetFramesFromAnimatedGIF(Image IMG) { 
        List<Image> gif_frames = new List<Image>();
        int Length = IMG.GetFrameCount(System.Drawing.Imaging.FrameDimension.Time);

        for (int i = 0; i < Length; i++)
        {
            IMG.SelectActiveFrame(System.Drawing.Imaging.FrameDimension.Time, i);
            gif_frames.Add(new Bitmap(IMG));
        }

        return gif_frames.ToArray();
    }
}

class Test {
    public static void Main(string[] args) {
        var filename = args[0];
        var procfile = new Bitmap(args[0]);
        var imgs = GifProcess.GetFramesFromAnimatedGIF(procfile);
        GifProcess.printToTerminal(imgs);
        // var baseImage = new ImageProccess(args[0]);
        // baseImage.printToTerminal();
    }
}