function formatInput() {
    const input = document.getElementById('userInput').value;

    // Split the text into paragraphs by double newlines
    const paragraphs = input.split(/\n\n/);

    // Convert each paragraph into a <p> element
    const formattedText = paragraphs
      .map((paragraph) => `<p>${paragraph.trim()}</p>`)
      .join('');

    // Inject the formatted text into the output div
    document.getElementById('output').innerHTML = formattedText;
  }