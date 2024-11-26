      // Initialize the Quill editor
      const quill = new Quill('#editor-container', {
        theme: 'snow',
        placeholder: 'Write something amazing...',
      });
  
      // Handle the "Show Output" button
      document.getElementById('submit-button').addEventListener('click', () => {
        // Get the formatted HTML content from the Quill editor
        const htmlContent = quill.root.innerHTML;
  
        // Render the formatted content in the output container
        document.getElementById('output-container').innerHTML = htmlContent;
  
        // Display the raw HTML in the preformatted block
        document.getElementById('html-output').textContent = htmlContent;
      });